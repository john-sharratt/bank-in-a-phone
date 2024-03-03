use egui::Ui;
use immutable_bank_model::bank_id::BankId;

use crate::{render::Mode, state::local_app::{FocusOn, LocalSession}, LocalApp};

impl LocalApp {
    pub fn login_bank(&mut self, ui: &mut Ui, frame: &mut eframe::Frame) {
        let bank_id = BankId::from(self.username.clone());
        let password_hash = self.compute_password_hash();
        if let Some(bank) = self.banks.get_mut(&bank_id) {
            self.password.clear();
            self.confirm_password.clear();
            self.focus_on.replace(FocusOn::Password);
            if bank.password != password_hash {
                self.show_dialog(ui, "Forbidden", "Invalid username or password");
                self.session.take();
                return;
            }
        } else {
            self.show_dialog(ui, "Forbidden", "No local bank of this name on the device");
            self.session.take();
            return;
        }

        self.session
            .replace(LocalSession::new(self.username.clone()));
        self.mode = Mode::Summary;

        self.save_state(frame);
    }
}
