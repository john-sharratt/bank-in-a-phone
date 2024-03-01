use rand::RngCore;

use crate::{ledger::Ledger, ledger_entry::LedgerEntry, ledger_type::LedgerType};

impl Ledger {
    pub fn add(&mut self, entry: LedgerType) {
        self.entries.push(Self::new_entry(entry));
    }

    pub fn add_with_id(&mut self, entry_id: u64, entry: LedgerType) {
        self.entries.push(LedgerEntry {
            id: entry_id,
            entry,
        });
    }

    pub fn new_entry(entry: LedgerType) -> LedgerEntry {
        let mut rand = rand::thread_rng();
        LedgerEntry {
            id: rand.next_u64(),
            entry,
        }
    }
}
