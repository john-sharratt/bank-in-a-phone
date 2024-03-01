use immutable_bank_model::{ledger_entry::LedgerEntry, ledger_type::LedgerType};

use crate::LocalApp;

impl LocalApp {
    pub fn try_finish(&mut self, and_then: impl FnOnce(&mut Self) -> ()) {
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
                    if Some(bank.owner.as_str()) == self.bank.as_ref().map(|b| b.owner.as_str()) {
                        log::info!("This bank updated");
                        self.bank.replace(bank.clone());
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
                self.show_error("Operation Failed", err);
            }
            None => {}
        }
    }
}
