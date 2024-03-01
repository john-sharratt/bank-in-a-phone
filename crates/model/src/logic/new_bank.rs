use rand::RngCore;

use crate::{
    account::{AccountRef, AccountType},
    bank::Bank,
    header::LedgerHeader,
    ledger::Ledger,
    ledger_type::LedgerEntry,
    transaction::Transaction,
};

impl Ledger {
    pub fn new_bank(&mut self, header: LedgerHeader, mut bank: Bank) {
        let mut rand = rand::thread_rng();

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

        self.add_with_header(
            LedgerHeader {
                id: header.id,
                signature: rand.next_u64(),
            },
            LedgerEntry::Transfer {
                transaction: Transaction {
                    from: AccountRef::Foreign {
                        bank: "John".to_string(),
                        account: AccountType::Printer,
                    },
                    to: AccountRef::Local {
                        account: AccountType::Wallet,
                    },
                    description: "Donation from money printer".to_string(),
                    amount_cents: 1_000_000_00,
                },
                local_bank: bank.owner.clone(),
            },
        );

        self.add_with_header(
            LedgerHeader {
                id: header.id,
                signature: rand.next_u64(),
            },
            LedgerEntry::Transfer {
                transaction: Transaction {
                    from: AccountRef::Local {
                        account: AccountType::Wallet,
                    },
                    to: AccountRef::Local {
                        account: AccountType::Savings,
                    },
                    description: "Squirreling some money away".to_string(),
                    amount_cents: 999_000_00,
                },
                local_bank: bank.owner.clone(),
            },
        );

        self.add_with_header(header, LedgerEntry::NewBank(bank.clone()));
    }
}
