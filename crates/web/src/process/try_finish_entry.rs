use egui::Ui;
use immutable_bank_model::ledger_type::LedgerEntry;

use crate::LocalApp;

impl LocalApp {
    pub fn try_finish(&mut self, ui: &mut Ui, and_then: impl FnOnce(&mut Self) -> ()) {
        let pending = match self.pending.clone() {
            Some(pending) => pending,
            None => {
                return;
            }
        };

        self.poll();

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
