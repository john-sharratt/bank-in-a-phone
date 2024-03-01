use immutable_bank_model::{bank::Bank, ledger_type::LedgerType};

use crate::LocalApp;

impl LocalApp {
    pub fn new_bank(&mut self, frame: &mut eframe::Frame) {
        let bank = Bank::new(self.username.clone());
        self.bank.replace(bank.clone());

        if let Err(err) = self.start_entry(LedgerType::NewBank(bank)) {
            self.show_error("Failed to create new bank", err);
            self.bank.take();
            return;
        }

        self.save_state(frame);
    }
}
