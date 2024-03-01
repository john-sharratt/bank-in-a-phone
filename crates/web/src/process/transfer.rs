use immutable_bank_model::{
    account::AccountRef, ledger_type::LedgerType, transaction::Transaction,
};

use crate::LocalApp;

impl LocalApp {
    pub fn transfer(&mut self, frame: &mut eframe::Frame) {
        let local_bank = match self.bank.as_ref().map(|b| b.owner.clone()) {
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
                to: AccountRef::Local {
                    account: self.to_account,
                },
                description: self.description.clone(),
                amount_cents: self.transfer_amount,
            },
        };

        if let Err(err) = self.start_entry(transfer) {
            self.show_error("Failed to create new bank", err);
            self.bank.take();
            return;
        }

        self.save_state(frame);
    }
}
