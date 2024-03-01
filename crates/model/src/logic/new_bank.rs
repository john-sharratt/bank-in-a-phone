use crate::{
    account::{AccountRef, AccountType},
    bank::Bank,
    ledger::Ledger,
    ledger_type::LedgerType,
    transaction::Transaction,
};

impl Ledger {
    pub fn new_bank(&mut self, entry_id: u64, mut bank: Bank) {
        self.add(LedgerType::Transfer {
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
        });

        self.add(LedgerType::Transfer {
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
        });

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

        self.add_with_id(entry_id, LedgerType::UpdateBank(bank.clone()));
    }
}
