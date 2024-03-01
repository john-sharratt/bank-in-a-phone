use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    account::AccountRef, header::LedgerHeader, ledger_type::LedgerEntry, transaction::Transaction,
};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Ledger {
    pub entries: BTreeMap<LedgerHeader, LedgerEntry>,
}

impl Ledger {
    pub fn entries_for(&self, name: &str) -> Vec<(&LedgerHeader, &LedgerEntry)> {
        self.entries
            .iter()
            .filter_map(|(h, e)| match e {
                LedgerEntry::Transfer {
                    local_bank,
                    transaction,
                } => {
                    if local_bank.as_str() == name {
                        Some((h, e))
                    } else if let AccountRef::Foreign { bank, .. } = &transaction.from {
                        if bank.as_str() == name {
                            Some((h, e))
                        } else {
                            None
                        }
                    } else if let AccountRef::Foreign { bank, .. } = &transaction.to {
                        if bank.as_str() == name {
                            Some((h, e))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                LedgerEntry::NewBank(bank) => {
                    if bank.owner.as_str() == name {
                        Some((h, e))
                    } else {
                        None
                    }
                }
                LedgerEntry::UpdateBank(bank) => {
                    if bank.owner.as_str() == name {
                        Some((h, e))
                    } else {
                        None
                    }
                }
                LedgerEntry::Error(_) => None,
            })
            .collect::<Vec<_>>()
    }

    pub fn transactions_for(&self, name: &str) -> Vec<&Transaction> {
        self.entries_for(name)
            .into_iter()
            .filter_map(|(_, e)| {
                if let LedgerEntry::Transfer { transaction, .. } = e {
                    Some(transaction)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }
}
