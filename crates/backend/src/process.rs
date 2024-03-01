use immutable_bank_model::{header::LedgerMessage, ledger_type::LedgerEntry};

use crate::general_state::GeneralStateInner;

impl GeneralStateInner {
    pub fn process(&mut self, msg: LedgerMessage) {
        if self.ledger.entries.contains_key(&msg.header) {
            tracing::warn!("Duplicate message: {:?}", msg.header);
            return;
        }

        tracing::info!("Received Msg: {:?}", msg);
        match msg.entry.clone() {
            LedgerEntry::NewBank(bank) => {
                if !self.existing_banks.contains(&bank.owner) {
                    let owner = bank.owner.clone();
                    self.ledger.new_bank(msg.header, bank);
                    self.existing_banks.insert(owner);
                } else {
                    self.ledger.entries.insert(
                        msg.header,
                        LedgerEntry::Error(format!("Bank already exist ({})", bank.owner)),
                    );
                }
            }
            LedgerEntry::UpdateBank(bank) => {
                if self.existing_banks.contains(&bank.owner) {
                    self.ledger.save_bank(msg.header, bank);
                } else {
                    self.ledger.entries.insert(
                        msg.header,
                        LedgerEntry::Error(format!("Foreign bank does not exist ({})", bank.owner)),
                    );
                }
            }
            LedgerEntry::Transfer {
                local_bank,
                transaction,
            } => {
                if self.existing_banks.contains(&local_bank) {
                    match self
                        .ledger
                        .transfer(msg.header.clone(), local_bank, transaction)
                    {
                        Ok(()) => {}
                        Err(err) => {
                            self.ledger
                                .entries
                                .insert(msg.header, LedgerEntry::Error(err.to_string()));
                        }
                    }
                } else {
                    self.ledger.entries.insert(
                        msg.header,
                        LedgerEntry::Error(format!("Foreign bank does not exist ({})", local_bank)),
                    );
                }
            }
            LedgerEntry::Error(err) => {
                tracing::debug!("ledger error (id={}) - {}", msg.header.id, err);
                self.ledger
                    .entries
                    .insert(msg.header, LedgerEntry::Error(err));
            }
        }
    }
}
