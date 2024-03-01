use immutable_bank_model::{ledger_entry::LedgerEntry, ledger_type::LedgerType};
use rand::RngCore;

use crate::LocalApp;

impl LocalApp {
    pub fn start_entry(&mut self, entry: LedgerType) -> anyhow::Result<()> {
        let mut rand = rand::thread_rng();
        let entry = LedgerEntry {
            id: rand.next_u64(),
            entry,
        };

        let data = bincode::serialize(&entry)?;
        self.ws.send(data);

        self.pending.replace(entry.id);

        Ok(())
    }
}
