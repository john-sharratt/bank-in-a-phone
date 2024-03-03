#![allow(unused)]
use rand::RngCore;

use crate::{
    account::{AccountRef, AccountType},
    bank::Bank,
    ledger::{Ledger, LedgerForBank, LedgerMessage},
    ledger_type::LedgerEntry,
    requests::new_bank::RequestNewBank,
    responses::create_bank::ResponseCreateBank,
    secret::LedgerSecret,
    transaction::Transaction,
};

impl Ledger {
    pub fn new_bank(
        &mut self,
        broker_secret: &LedgerSecret,
        req: RequestNewBank,
        mut on_msg: impl FnMut(&LedgerMessage),
    ) -> anyhow::Result<ResponseCreateBank> {
        let mut bank = req.bank;
        let mut rand = rand::thread_rng();

        // If it already exists then fail
        let bank_id = bank.id();
        if let Some(ledger) = self.banks.get(&bank_id) {
            if ledger.bank_secret != req.secret {
                return Ok(ResponseCreateBank::AlreadyExists {
                    err_msg: format!("A bank with this ID already exists"),
                });
            }
        }

        // Put some money in the bank
        for acc in bank.accounts.iter_mut() {
            match acc.type_ {
                AccountType::Wallet => {
                    acc.balance_cents = 1_000_00;
                }
                AccountType::Savings => {
                    acc.balance_cents = 999_000_00;
                }
                AccountType::Printer => {}
            }
        }

        // Insert the secret into the ledger
        self.banks.insert(
            bank_id.clone(),
            LedgerForBank {
                broker_secret: broker_secret.clone(),
                bank_secret: req.secret.clone(),
                entries: Default::default(),
            },
        );

        // Create the entry and the signature using the secret then add it
        // to the journal
        let entry = LedgerEntry::NewBank {
            bank_secret: req.secret.clone(),
            bank: bank.clone(),
        };
        let signature = req.secret.sign(&entry);
        self.add(bank_id, entry, signature, Some(&mut on_msg))?;

        Ok(ResponseCreateBank::Created { bank })
    }
}
