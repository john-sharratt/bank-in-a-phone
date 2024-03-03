use egui::Ui;
use immutable_bank_model::{
    account::AccountRef, requests::transfer::RequestTransfer,
    responses::transfer::ResponseTransfer, transaction::Transaction,
};

use crate::{render::Mode, LocalApp};

impl LocalApp {
    pub fn transfer(&mut self, _ui: &mut Ui, _frame: &mut eframe::Frame) {
        let local_bank = match self.bank_with_secrets() {
            Some(a) => a,
            None => {
                return;
            }
        };
        let trans = Transaction {
            from: AccountRef {
                bank: local_bank.bank_id.clone(),
                account: self.from_account,
            },
            to: if !self.to_bank.is_empty() {
                AccountRef {
                    bank: self.to_bank.clone().into(),
                    account: self.to_account,
                }
            } else {
                AccountRef {
                    bank: local_bank.bank_id.clone(),
                    account: self.to_account,
                }
            },
            description: self.description.clone(),
            amount_cents: self.transfer_amount,
        };
        let request = RequestTransfer {
            signature: local_bank.secret.sign(&trans),
            trans,
        };

        self.start_post(
            "transfer",
            request,
            |res: ResponseTransfer, app, frame| match res {
                ResponseTransfer::InvalidBank { bank_id } => {
                    app.show_dialog_lite(
                        "Invalid bank",
                        &format!("BankID is invalid ({})", bank_id.as_str()),
                    );
                }
                ResponseTransfer::InvalidSignature => {
                    app.show_dialog_lite(
                        "Invalid transfer signature",
                        &format!("Signature attached to request is invalid"),
                    );
                }
                ResponseTransfer::InvalidAccount { bank_id, account } => {
                    app.show_dialog_lite(
                        "Invalid account",
                        &format!(
                            "Bank account ({account:?}) is invalid ({})",
                            bank_id.as_str()
                        ),
                    );
                }
                ResponseTransfer::InsufficientFunds { available, .. } => app.show_dialog_lite(
                    "Insufficient funds",
                    &format!("Not enough funds to execute the transfer (available={available})"),
                ),
                ResponseTransfer::Transferred => {
                    app.mode = Mode::Summary;
                    app.save_state(frame);
                }
            },
        );
    }
}
