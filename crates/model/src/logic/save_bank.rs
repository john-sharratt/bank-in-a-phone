use crate::{bank::Bank, ledger::Ledger, ledger_type::LedgerType};

impl Ledger {
    pub fn save_bank(&mut self, entry_id: u64, bank: Bank) {
        self.add_with_id(entry_id, LedgerType::UpdateBank(bank.clone()));
    }
}
