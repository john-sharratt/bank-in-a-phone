use egui::Ui;

use crate::{render::Mode, LocalApp};

impl LocalApp {
    pub fn login_bank(&mut self, ui: &mut Ui, frame: &mut eframe::Frame) {
        let password_hash = self.compute_password_hash();
        if let Some(bank) = self.banks.get_mut(&self.username) {
            if bank.password_hash != password_hash {
                self.show_dialog(ui, "Forbidden", "Invalid username or password");
                self.session.take();
                return;
            }
        } else {
            self.show_dialog(ui, "Forbidden", "No local bank of this name on the device");
            self.session.take();
            return;
        }

        self.session.replace(self.username.clone());
        self.mode = Mode::Summary;

        self.save_state(frame);
    }
}
