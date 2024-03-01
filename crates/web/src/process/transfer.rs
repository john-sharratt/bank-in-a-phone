use egui::Ui;
use immutable_bank_model::{
    account::AccountRef, ledger_type::LedgerType, transaction::Transaction,
};

use crate::LocalApp;

impl LocalApp {
    pub fn transfer(&mut self, ui: &mut Ui, frame: &mut eframe::Frame) {
        let local_bank = match self.bank().map(|s| s.owner.clone()) {
            Some(a) => a,
            None => {
                return;
            }
        };
        let transfer = LedgerType::Transfer {
            local_bank,
            transaction: Transaction {
                from: AccountRef::Local {
                    account: self.from_account,
                },
                to: if self.to_user.is_empty() {
                    AccountRef::Local {
                        account: self.to_account,
                    }
                } else {
                    AccountRef::Foreign {
                        bank: self.to_user.clone(),
                        account: self.to_account,
                    }
                },
                description: self.description.clone(),
                amount_cents: self.transfer_amount,
            },
        };

        if let Err(err) = self.start_entry(transfer) {
            self.show_error(ui, "Failed to transfer funds", err);
            self.session.take();
            return;
        }

        self.save_state(frame);
    }
}
