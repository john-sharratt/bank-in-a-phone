use egui::Ui;
use immutable_bank_model::{
    bank::Bank, bank_id::BankId, password_hash::PasswordHash, requests::new_bank::RequestNewBank,
    responses::create_bank::ResponseCreateBank, secret::LedgerSecret,
};

use crate::{
    render::Mode,
    state::local_app::{BankWithSecrets, LocalSession},
    LocalApp,
};

impl LocalApp {
    pub fn compute_password_hash(&self) -> PasswordHash {
        PasswordHash::from_password(&self.username, &self.password)
    }

    pub fn new_bank(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        let bank_id = BankId::from(self.username.clone());
        if self.banks.contains_key(&bank_id) {
            self.show_dialog(
                ui,
                "Bank already exists",
                "Bank already exists locally, login instead",
            );
            self.session.take();
            return;
        }
        let secret = LedgerSecret::new();
        let password_hash = self.compute_password_hash();

        let bank = Bank::new(self.username.clone().into(), password_hash.clone());
        self.start_post(
            "new-bank",
            RequestNewBank {
                secret,
                bank: bank.clone(),
            },
            move |res: ResponseCreateBank, app, frame| match res {
                ResponseCreateBank::Created { .. } => {
                    app.banks.insert(
                        bank_id.clone(),
                        BankWithSecrets {
                            bank_id,
                            secret: secret.clone(),
                            password: password_hash,
                        },
                    );
                    app.session.replace(LocalSession::new(bank.owner.clone()));
                    app.mode = Mode::Summary;
                    app.save_state(frame);
                }
                ResponseCreateBank::AlreadyExists { err_msg } => {
                    app.show_dialog_lite("Bank already exists", &err_msg);
                }
            },
        );
    }
}
