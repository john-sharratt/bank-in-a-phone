use immutable_bank_model::{header::LedgerMessage, ledger_type::LedgerEntry};

use crate::LocalApp;

impl LocalApp {
    pub fn start_entry(&mut self, entry: LedgerEntry) -> anyhow::Result<()> {
        let msg = LedgerMessage {
            header: self.ledger.new_header(),
            entry,
        };

        let data = bincode::serialize(&msg)?;
        self.ws.send(data);

        self.pending.replace(msg.header);

        Ok(())
    }
}
