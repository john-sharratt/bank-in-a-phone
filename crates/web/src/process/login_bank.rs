use egui::Ui;
use immutable_bank_model::{
    bank_id::BankId, requests::copy_bank::RequestCopyBank, responses::copy_bank::ResponseCopyBank,
};

use crate::{
    render::Mode,
    state::local_app::{BankWithSecrets, FocusOn, LocalSession},
    LocalApp,
};

impl LocalApp {
    pub fn login_bank(&mut self, ui: &mut Ui, frame: &mut eframe::Frame) {
        let bank_id = BankId::from(self.username.to_lowercase());
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
            self.start_post(
                "copy-bank",
                RequestCopyBank {
                    password: password_hash.clone(),
                    bank: bank_id.clone(),
                },
                move |res: ResponseCopyBank, app: &mut LocalApp, frame: &mut eframe::Frame| {
                    match res {
                        ResponseCopyBank::Copied { ledger } => {
                            app.banks.insert(
                                bank_id.clone(),
                                BankWithSecrets {
                                    bank_id: bank_id.clone(),
                                    secret: ledger.bank_secret.clone(),
                                    password: password_hash,
                                },
                            );
                            app.ledger.banks.insert(bank_id.clone(), ledger);
                            app.session.replace(LocalSession::new(bank_id.clone()));
                            app.mode = Mode::Summary;
                            app.save_state(frame);
                        }
                        ResponseCopyBank::Denied { err_msg } => {
                            app.show_dialog_lite("Forbidden", &err_msg);
                            app.session.take();
                            app.mode = Mode::Login;
                        }
                        ResponseCopyBank::DoesNotExist { bank_id } => {
                            app.show_dialog_lite(
                                "Invalid Bank",
                                &format!("Bank does not exist - {:?}", bank_id),
                            );
                            app.session.take();
                            app.mode = Mode::Login;
                        }
                    }
                },
            );

            return;
        }

        self.session
            .replace(LocalSession::new(self.username.to_lowercase()));
        self.mode = Mode::Summary;

        self.save_state(frame);
    }
}
