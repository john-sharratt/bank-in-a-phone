use std::collections::HashMap;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::{
    account::{Account, AccountType},
    bank::Bank,
    bank_id::BankId,
    ledger_type::LedgerEntry,
    secret::LedgerSecret,
    signature::LedgerSignature,
    transaction::Transaction,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LedgerForBank {
    pub broker_secret: LedgerSecret,
    pub bank_secret: LedgerSecret,
    pub entries: IndexMap<LedgerSignature, LedgerMessage>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LedgerBrokerHeader {
    pub bank_id: BankId,
    pub prev_signature: LedgerSignature,
    pub bank_signature: LedgerSignature,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LedgerMessage {
    pub header: LedgerBrokerHeader,
    pub broker_signature: LedgerSignature,
    pub entry: LedgerEntry,
}

impl LedgerForBank {
    pub fn bank(&self) -> Option<&Bank> {
        self.entries
            .iter()
            .into_iter()
            .rev()
            .filter_map(|(_, m)| {
                if let LedgerEntry::UpdateBank(bank) = &m.entry {
                    Some(bank)
                } else if let LedgerEntry::NewBank { bank, .. } = &m.entry {
                    Some(bank)
                } else {
                    None
                }
            })
            .next()
    }

    pub fn account(&self, account: AccountType) -> Option<&Account> {
        self.bank()
            .and_then(|bank| bank.accounts.iter().filter(|a| a.type_ == account).next())
    }

    pub fn tail_signature(&self) -> LedgerSignature {
        self.entries
            .last()
            .map(|(e, _)| e.clone())
            .unwrap_or_else(|| LedgerSignature::ZERO)
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Ledger {
    pub banks: HashMap<BankId, LedgerForBank>,
}

impl Ledger {
    pub fn ledger_for<ID>(&self, bank_id: ID) -> Option<&LedgerForBank>
    where
        ID: Into<BankId>,
    {
        let bank_id: BankId = bank_id.into();
        self.banks.get(&bank_id).map(|ledger| ledger)
    }

    pub fn ledger_mut_for<ID>(&mut self, bank_id: ID) -> Option<&mut LedgerForBank>
    where
        ID: Into<BankId>,
    {
        let bank_id: BankId = bank_id.into();
        self.banks.get_mut(&bank_id).map(|ledger| ledger)
    }

    pub fn entries_for<ID>(&self, bank_id: ID) -> Vec<&LedgerMessage>
    where
        ID: Into<BankId>,
    {
        self.ledger_for(bank_id)
            .map(|b| b.entries.iter().map(|e| e.1).collect::<Vec<_>>())
            .unwrap_or_default()
    }

    pub fn entries_mut_for<ID>(&mut self, bank_id: ID) -> Vec<&mut LedgerMessage>
    where
        ID: Into<BankId>,
    {
        self.ledger_mut_for(bank_id)
            .map(|b| b.entries.iter_mut().map(|e| e.1).collect::<Vec<_>>())
            .unwrap_or_default()
    }

    pub fn bank<ID>(&self, bank_id: ID) -> Option<&Bank>
    where
        ID: Into<BankId>,
    {
        self.ledger_for(bank_id).and_then(|b| b.bank())
    }

    pub fn transactions_for<ID>(&self, bank_id: ID) -> Vec<&Transaction>
    where
        ID: Into<BankId>,
    {
        self.entries_for(bank_id)
            .into_iter()
            .filter_map(|e| {
                if let LedgerEntry::Transaction { transaction, .. } = &e.entry {
                    Some(transaction)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }
}
