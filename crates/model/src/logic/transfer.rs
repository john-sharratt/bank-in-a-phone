use crate::{
    account::AccountRef, bank::Bank, ledger::Ledger, ledger_type::LedgerType,
    transaction::Transaction,
};

impl Ledger {
    pub fn transfer(
        &mut self,
        entry_id: u64,
        local_bank: String,
        trans: Transaction,
    ) -> anyhow::Result<()> {
        let mut from_bank;
        let mut to_bank;

        match trans.from.clone() {
            AccountRef::Local { account } => {
                if let Some(bank) = self.find_bank(local_bank.clone()) {
                    from_bank = bank.clone();
                    match from_bank.find_account(account) {
                        Some(account) => {
                            if trans.amount_cents > account.balance_cents {
                                return Err(anyhow::anyhow!(
                                    "Insufficient funds - {} vs {}",
                                    trans.amount_cents,
                                    account.balance_cents,
                                ));
                            }
                            account.balance_cents -= trans.amount_cents;
                        }
                        None => {
                            return Err(anyhow::anyhow!(
                                "Bank does not this account type - {}",
                                account
                            ));
                        }
                    }
                } else {
                    return Err(anyhow::anyhow!("Bank does not exist - {}", local_bank));
                }
            }
            AccountRef::Foreign { username, account } => {
                if let Some(bank) = self.find_bank(username) {
                    from_bank = bank.clone();
                    match from_bank.find_account(account) {
                        Some(account) => {
                            if trans.amount_cents > account.balance_cents {
                                return Err(anyhow::anyhow!(
                                    "Insufficient funds - {} vs {}",
                                    trans.amount_cents,
                                    account.balance_cents,
                                ));
                            }
                            account.balance_cents -= trans.amount_cents;
                        }
                        None => {
                            return Err(anyhow::anyhow!(
                                "Bank does not this account type - {}",
                                account
                            ));
                        }
                    }
                } else {
                    return Err(anyhow::anyhow!("Bank does not exist - {}", local_bank));
                }
            }
        }

        match trans.to.clone() {
            AccountRef::Local { account } => {
                if let Some(bank) = self.find_bank(local_bank.clone()) {
                    to_bank = if bank.owner == from_bank.owner {
                        from_bank.clone()
                    } else {
                        bank.clone()
                    };
                    match to_bank.find_account(account) {
                        Some(account) => {
                            account.balance_cents += trans.amount_cents;
                        }
                        None => {
                            return Err(anyhow::anyhow!(
                                "Bank does not this account type - {}",
                                account
                            ));
                        }
                    }
                } else {
                    return Err(anyhow::anyhow!("Bank does not exist - {}", local_bank));
                }
            }
            AccountRef::Foreign { username, account } => {
                if let Some(bank) = self.find_bank(username) {
                    to_bank = if bank.owner == from_bank.owner {
                        from_bank.clone()
                    } else {
                        bank.clone()
                    };
                    match to_bank.find_account(account) {
                        Some(account) => {
                            account.balance_cents += trans.amount_cents;
                        }
                        None => {
                            return Err(anyhow::anyhow!(
                                "Bank does not this account type - {}",
                                account
                            ));
                        }
                    }
                } else {
                    return Err(anyhow::anyhow!("Bank does not exist - {}", local_bank));
                }
            }
        }

        if from_bank.owner != to_bank.owner {
            self.add(LedgerType::UpdateBank(from_bank));
        }
        self.add(LedgerType::UpdateBank(to_bank));

        self.add_with_id(
            entry_id,
            LedgerType::Transfer {
                local_bank,
                transaction: trans,
            },
        );
        Ok(())
    }

    pub fn find_bank(&mut self, name: String) -> Option<Bank> {
        self.entries
            .iter()
            .rev()
            .filter_map(|e| match &e.entry {
                LedgerType::NewBank(bank) | LedgerType::UpdateBank(bank) => Some(bank),
                _ => None,
            })
            .find(|b| b.owner == name)
            .cloned()
    }
}
