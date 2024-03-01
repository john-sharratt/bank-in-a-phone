use immutable_bank_model::{ledger_entry::LedgerEntry, ledger_type::LedgerType};

use crate::general_state::GeneralStateInner;

impl GeneralStateInner {
    pub fn process(&mut self, entry: LedgerEntry) {
        match entry.entry.clone() {
            LedgerType::NewBank(bank) => {
                if !self.existing_banks.contains(&bank.owner) {
                    let owner = bank.owner.clone();
                    self.ledger.new_bank(entry.id, bank);
                    self.existing_banks.insert(owner);
                } else {
                    self.ledger.entries.push(LedgerEntry {
                        id: entry.id,
                        entry: LedgerType::Error(format!("Bank already exist ({})", bank.owner)),
                    });
                }
            }
            LedgerType::UpdateBank(bank) => {
                if self.existing_banks.contains(&bank.owner) {
                    self.ledger.save_bank(entry.id, bank);
                } else {
                    self.ledger.entries.push(LedgerEntry {
                        id: entry.id,
                        entry: LedgerType::Error(format!(
                            "Foreign bank does not exist ({})",
                            bank.owner
                        )),
                    });
                }
            }
            LedgerType::Transfer {
                local_bank,
                transaction,
            } => {
                if self.existing_banks.contains(&local_bank) {
                    match self.ledger.transfer(entry.id, local_bank, transaction) {
                        Ok(()) => {}
                        Err(err) => {
                            self.ledger.entries.push(LedgerEntry {
                                id: entry.id,
                                entry: LedgerType::Error(err.to_string()),
                            });
                        }
                    }
                } else {
                    self.ledger.entries.push(LedgerEntry {
                        id: entry.id,
                        entry: LedgerType::Error(format!(
                            "Foreign bank does not exist ({})",
                            local_bank
                        )),
                    });
                }
            }
            LedgerType::Error(err) => {
                tracing::debug!("ledger error (id={}) - {}", entry.id, err);
                self.ledger.entries.push(LedgerEntry {
                    id: entry.id,
                    entry: LedgerType::Error(err),
                });
            }
        }
    }
}
