use rand::RngCore;

use crate::{header::LedgerHeader, ledger::Ledger, ledger_type::LedgerEntry};

impl Ledger {
    pub fn add(&mut self, entry: LedgerEntry) {
        let header = self.new_header();
        self.entries.insert(header, entry);
    }

    pub fn add_with_header(&mut self, header: LedgerHeader, entry: LedgerEntry) {
        self.entries.insert(header, entry);
    }

    pub fn new_header(&self) -> LedgerHeader {
        let mut rand = rand::thread_rng();
        LedgerHeader {
            id: self
                .entries
                .last_key_value()
                .map(|e| e.0.id + 1)
                .unwrap_or(1),
            signature: rand.next_u64(),
        }
    }
}
