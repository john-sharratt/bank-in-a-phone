use egui::Ui;
use immutable_bank_model::{bank::Bank, ledger_type::LedgerEntry};

use crate::{state::local_app::BankAndPassword, LocalApp};
use sha256::digest;

impl LocalApp {
    pub fn compute_password_hash(&self) -> String {
        digest(&format!(
            "seed:{},password:{}",
            self.username, self.password
        ))
    }
    pub fn new_bank(&mut self, ui: &mut Ui, frame: &mut eframe::Frame) {
        if self.banks.contains_key(&self.username) {
            self.show_dialog(
                ui,
                "Bank already exists",
                "Bank already exists locally, login instead",
            );
            self.session.take();
            return;
        }
        let password_hash = self.compute_password_hash();

        let bank = Bank::new(self.username.clone(), password_hash.clone());
        self.banks.insert(
            self.username.clone(),
            BankAndPassword {
                bank: bank.clone(),
                password_hash,
            },
        );
        self.session.replace(self.username.clone());

        if let Err(err) = self.start_entry(LedgerEntry::NewBank(bank)) {
            self.show_error(ui, "Failed to create new bank", err);
            self.session.take();
            return;
        }

        self.save_state(frame);
    }
}
