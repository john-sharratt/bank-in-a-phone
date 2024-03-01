use egui::Ui;
use immutable_bank_model::{header::LedgerMessage, ledger_type::LedgerEntry};

use crate::LocalApp;

impl LocalApp {
    pub fn try_finish(&mut self, ui: &mut Ui, and_then: impl FnOnce(&mut Self) -> ()) {
        let pending = match self.pending.clone() {
            Some(pending) => pending,
            None => {
                return;
            }
        };

        while let Some(msg) = self.ws.try_recv() {
            let msg: LedgerMessage = match bincode::deserialize(&msg) {
                Ok(e) => e,
                Err(err) => {
                    log::error!("failed to deserialize msg - {}", err);
                    continue;
                }
            };

            match &msg.entry {
                LedgerEntry::NewBank(bank) | LedgerEntry::UpdateBank(bank) => {
                    if let Some(b) = self.banks.get_mut(&bank.owner) {
                        log::info!("Local bank ({}) updated", bank.owner);
                        b.bank = bank.clone();
                    } else {
                        log::info!("Foreign bank ({}) updated", bank.owner);
                    }
                }
                LedgerEntry::Transfer { transaction, .. } => {
                    log::info!("Money Transfer {}->{}", transaction.from, transaction.to);
                }
                LedgerEntry::Error(err) => {
                    log::info!("Ledger Error - {}", err);
                }
            }

            self.ledger.entries.insert(msg.header, msg.entry);
        }

        let res = self.ledger.entries.get(&pending).map(|entry| match &entry {
            LedgerEntry::Error(err) => Err(anyhow::anyhow!("{}", err)),
            _ => Ok(()),
        });

        match res {
            Some(Ok(())) => {
                self.pending.take();
                and_then(self);
            }
            Some(Err(err)) => {
                self.pending.take();
                self.show_error(ui, "Operation Failed", err);
            }
            None => {}
        }
    }
}
