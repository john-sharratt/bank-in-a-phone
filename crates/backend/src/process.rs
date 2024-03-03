use immutable_bank_model::{
    ledger::{LedgerForBank, LedgerMessage},
    ledger_type::LedgerEntry,
    secret::LedgerSecret,
};

use crate::{general_state::GeneralStateInner, BROKER_SECRET};

impl GeneralStateInner {
    pub fn process(&mut self, msg: &LedgerMessage) -> bool {
        // Create the ledger entry
        let ledger = self
            .ledger
            .banks
            .entry(msg.header.bank_id.clone())
            .or_insert_with(|| LedgerForBank {
                broker_secret: BROKER_SECRET.clone(),
                bank_secret: LedgerSecret::new(),
                entries: Vec::new(),
            });

        // Check if the message is a duplicate
        if msg.header.index <= ledger.entries.len() as u64 {
            tracing::debug!("Duplicate message: {:?}", msg.entry);
            return false;
        }

        // We establish the bank secret if it is not already established
        if ledger.entries.is_empty() {
            if let LedgerEntry::NewBank { bank_secret, .. } = &msg.entry {
                ledger.bank_secret = bank_secret.clone();
            }
        }

        // It has to be the next entry in the list or we have split brain
        if ledger.entries.len() as u64 != msg.header.index {
            tracing::warn!(
                "Split brain: {} vs {}",
                ledger.entries.len(),
                msg.header.index
            );
            return false;
        }

        // Validate the message
        if ledger.bank_secret.sign(&msg.entry) != msg.header.bank_signature {
            tracing::warn!("Invalid bank signature: {:?}", msg.entry);
            return false;
        }
        if ledger.broker_secret.sign(&msg.header) != msg.broker_signature {
            tracing::warn!("Invalid bank signature: {:?}", msg.entry);
            return false;
        }

        // Insert the entry
        tracing::info!("Recorded entry: {:?}", msg.entry);
        ledger.entries.push(msg.clone());
        true
    }
}
