#![allow(unused)]
use rand::RngCore;

use crate::{
    account::{AccountRef, AccountType},
    bank::Bank,
    ledger::{Ledger, LedgerForBank, LedgerMessage},
    ledger_type::LedgerEntry,
    password_hash::PasswordHash,
    requests::{copy_bank::RequestCopyBank, new_bank::RequestNewBank},
    responses::{
        copy_bank::{Copied, ResponseCopyBank},
        create_bank::ResponseCreateBank,
    },
    secret::LedgerSecret,
    transaction::Transaction,
};

impl Ledger {
    pub fn copy_bank(&mut self, req: RequestCopyBank) -> anyhow::Result<ResponseCopyBank> {
        let mut bank_id = req.bank;
        let mut rand = rand::thread_rng();

        // Get the bank, if it does not exist then so be it
        let ledger = match self.banks.get(&bank_id) {
            Some(b) => b,
            None => {
                return Ok(ResponseCopyBank::DoesNotExist { bank_id });
            }
        };
        let bank = match ledger.bank() {
            Some(b) => b,
            None => {
                return Ok(ResponseCopyBank::DoesNotExist { bank_id });
            }
        };

        // Check the password
        if bank.password != req.password {
            return Ok(ResponseCopyBank::Denied {
                err_msg: format!("Invalid password."),
            });
        }

        // Send the ledger!
        Ok(ResponseCopyBank::Copied(Copied {
            bank_secret: ledger.bank_secret.clone(),
            entries: ledger.entries.values().cloned().collect(),
        }))
    }
}
