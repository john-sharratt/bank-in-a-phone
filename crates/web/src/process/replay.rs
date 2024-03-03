use crate::LocalApp;

impl LocalApp {
    pub fn replay(&mut self) -> anyhow::Result<()> {
        // Get all the banks that we need to replay (we only replay banks that we own)
        let banks = self.banks.keys().cloned().collect::<Vec<_>>();
        for bank in banks {
            // Get all the entries related to this bank and play them down the web socket
            let entries = self.ledger.entries_for(bank.as_str());
            for msg in entries {
                log::info!("Replaying message {:?}", msg);
                let data = bincode::serialize(&msg)?;

                let ws = match self.session.as_mut() {
                    Some(ws) => &mut ws.ws,
                    None => continue,
                };
                ws.send(data);
            }
        }
        Ok(())
    }
}
