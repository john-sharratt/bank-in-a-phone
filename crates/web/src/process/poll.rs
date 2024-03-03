use immutable_bank_model::{
    ledger::{LedgerForBank, LedgerMessage},
    ledger_type::LedgerEntry,
    secret::LedgerSecret,
};

use crate::LocalApp;

impl LocalApp {
    pub fn poll(&mut self) {
        if self.session.is_none() {
            return;
        }

        if self.last_reconnects != self.session.as_mut().unwrap().ws.reconnects() {
            if let Err(err) = self.replay() {
                log::warn!("Failed to replay local ledger - {}", err);
            }
            self.last_reconnects = self.session.as_mut().unwrap().ws.reconnects();
        }

        while let Some(msg) = self.session.as_mut().unwrap().ws.try_recv() {
            let msg: LedgerMessage = match bincode::deserialize(&msg) {
                Ok(e) => e,
                Err(err) => {
                    log::error!("failed to deserialize msg - {}", err);
                    continue;
                }
            };

            // Find the ledger for this message
            if let LedgerEntry::NewBank { bank_secret, .. } = &msg.entry {
                if self.ledger.banks.contains_key(&msg.header.bank_id) == false {
                    self.ledger
                        .banks
                        .entry(msg.header.bank_id.clone())
                        .or_insert_with(|| LedgerForBank {
                            broker_secret: LedgerSecret::new(),
                            bank_secret: bank_secret.clone(),
                            entries: Vec::new(),
                        });
                }
            }
            let ledger = match self.ledger.ledger_mut_for(msg.header.bank_id.clone()) {
                Some(ledger) => ledger,
                None => {
                    log::debug!("Missing bank for message {:?}", msg.entry);
                    continue;
                }
            };

            // Validate the bank signature
            let signature = ledger.bank_secret.sign(&msg.entry);
            if signature != msg.header.bank_signature {
                log::debug!("Invalid signature for message {:?}", msg.header);
                continue;
            }

            // Add it to the ledger, unless it already exists
            if msg.header.index < ledger.entries.len() as u64 {
                log::debug!("Duplicate message {:?}", msg.header);
            } else if ledger.entries.len() as u64 == msg.header.index {
                log::debug!("Received message {:?}", msg);
                ledger.entries.push(msg);
            } else {
                log::debug!("Split brain {:?}", msg.header);
            }
        }
    }
}
