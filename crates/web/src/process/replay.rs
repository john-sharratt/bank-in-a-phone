use immutable_bank_model::header::LedgerMessage;

use crate::LocalApp;

impl LocalApp {
    pub fn replay(&mut self) -> anyhow::Result<()> {
        // Get all the banks that we need to replay (we only replay banks that we own)
        let banks = self.banks.keys().cloned().collect::<Vec<_>>();
        for bank in banks {
            // Get all the entries related to this bank and play them down the web socket
            let entries = self.ledger.entries_for(bank.as_str());
            for (header, entry) in entries {
                let msg = LedgerMessage {
                    header: header.clone(),
                    entry: entry.clone(),
                };

                log::info!("Replaying message {:?}", msg);
                let data = bincode::serialize(&msg)?;
                self.ws.send(data);
            }
        }
        Ok(())
    }
}
