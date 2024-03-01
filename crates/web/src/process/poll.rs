use immutable_bank_model::{account::AccountRef, header::LedgerMessage, ledger_type::LedgerEntry};

use crate::LocalApp;

impl LocalApp {
    pub fn poll(&mut self) {
        if self.last_reconnects != self.ws.reconnects() {
            if let Err(err) = self.replay() {
                log::warn!("Failed to replay local ledger - {}", err);
            }
            self.last_reconnects = self.ws.reconnects();
        }

        while let Some(msg) = self.ws.try_recv() {
            let msg: LedgerMessage = match bincode::deserialize(&msg) {
                Ok(e) => e,
                Err(err) => {
                    log::error!("failed to deserialize msg - {}", err);
                    continue;
                }
            };

            if self.ledger.entries.contains_key(&msg.header) {
                log::debug!("Duplicate message {:?}", msg.header);
                continue;
            }

            match &msg.entry {
                LedgerEntry::NewBank(bank) | LedgerEntry::UpdateBank(bank) => {
                    if let Some(b) = self.banks.get_mut(&bank.owner) {
                        log::info!("Local bank ({}) updated", bank.owner);
                        b.bank = bank.clone();
                    } else {
                        log::info!("Foreign bank ({}) updated", bank.owner);
                        continue;
                    }
                }
                LedgerEntry::Transfer {
                    transaction,
                    local_bank,
                } => {
                    let is_local = if self.banks.contains_key(local_bank) {
                        true
                    } else if let AccountRef::Foreign { bank, .. } = &transaction.from {
                        self.banks.contains_key(bank)
                    } else if let AccountRef::Foreign { bank, .. } = &transaction.to {
                        self.banks.contains_key(bank)
                    } else {
                        false
                    };
                    if is_local {
                        log::info!("Money Transfer {}->{}", transaction.from, transaction.to);
                    } else {
                        log::info!(
                            "Foreign Money Transfer {}->{}",
                            transaction.from,
                            transaction.to
                        );
                        continue;
                    }
                }
                LedgerEntry::Error(err) => {
                    if Some(msg.header) == self.pending {
                        log::info!("Ledger Error - {}", err);
                    } else {
                        log::info!("Foreign Ledger Error - {}", err);
                        continue;
                    }
                }
            }

            self.ledger.entries.insert(msg.header, msg.entry);
        }
    }
}
