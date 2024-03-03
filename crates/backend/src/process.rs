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
                bank_secret: if let LedgerEntry::NewBank { bank_secret, .. } = &msg.entry {
                    bank_secret.clone()
                } else {
                    LedgerSecret::new()
                },
                entries: Default::default(),
            });

        // Check if the message is a duplicate
        if ledger.entries.contains_key(&msg.broker_signature) {
            tracing::debug!("Duplicate message: {}", msg.entry);
            return false;
        }

        // It has to be the next entry in the list or we have split brain
        if ledger.tail_signature() != msg.header.prev_signature {
            tracing::warn!(
                "Split brain: {} vs {}",
                ledger.tail_signature(),
                msg.broker_signature
            );
            return false;
        }

        // Validate the message
        if ledger.bank_secret.sign(&msg.entry) != msg.header.bank_signature {
            tracing::warn!("Invalid bank signature: {}", msg.entry);
            return false;
        }
        if ledger.broker_secret.sign(&msg.header) != msg.broker_signature {
            tracing::warn!("Invalid bank signature: {}", msg.entry);
            return false;
        }

        // Insert the entry
        tracing::info!("Recovered entry: {}", msg.entry);
        ledger
            .entries
            .insert(msg.broker_signature.clone(), msg.clone());
        true
    }
}
