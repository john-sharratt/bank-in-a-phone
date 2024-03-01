use crate::{bank::Bank, header::LedgerHeader, ledger::Ledger, ledger_type::LedgerEntry};

impl Ledger {
    pub fn save_bank(&mut self, header: LedgerHeader, bank: Bank) {
        self.add_with_header(header, LedgerEntry::UpdateBank(bank.clone()));
    }
}
