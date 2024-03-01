use egui::Ui;
use immutable_bank_model::{ledger_entry::LedgerEntry, ledger_type::LedgerType};

use crate::LocalApp;

impl LocalApp {
    pub fn try_finish(&mut self, ui: &mut Ui, and_then: impl FnOnce(&mut Self) -> ()) {
        let id = match self.pending.clone() {
            Some(id) => id,
            None => {
                return;
            }
        };

        while let Some(msg) = self.ws.try_recv() {
            let entry: LedgerEntry = match bincode::deserialize(&msg) {
                Ok(e) => e,
                Err(err) => {
                    log::error!("failed to deserialize msg - {}", err);
                    continue;
                }
            };

            match &entry.entry {
                LedgerType::NewBank(bank) | LedgerType::UpdateBank(bank) => {
                    if let Some(b) = self.banks.get_mut(&bank.owner) {
                        log::info!("Local bank ({}) updated", bank.owner);
                        b.bank = bank.clone();
                    }
                }
                LedgerType::Transfer { .. } => {}
                LedgerType::Error(_) => {}
            }

            self.ledger.entries.push(entry);
        }

        let res = self
            .ledger
            .entries
            .iter()
            .find(|e| e.id == id)
            .map(|entry| match &entry.entry {
                LedgerType::Error(err) => Err(anyhow::anyhow!("{}", err)),
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
