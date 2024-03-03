use crate::{
    bank_id::BankId,
    ledger::{Ledger, LedgerBrokerHeader, LedgerMessage},
    ledger_type::LedgerEntry,
    signature::LedgerSignature,
};

impl Ledger {
    pub fn add<ID>(
        &mut self,
        bank_id: ID,
        entry: LedgerEntry,
        verify_signature: LedgerSignature,
        on_msg: Option<&mut impl FnMut(&LedgerMessage)>,
    ) -> anyhow::Result<()>
    where
        ID: Into<BankId>,
    {
        // Get the ledger for this bank
        let bank_id: BankId = bank_id.into();
        let ledger = self
            .banks
            .get_mut(&bank_id)
            .ok_or(anyhow::anyhow!("The bank is unknown to this ledger"))?;

        // Build the signature
        let bank_signature = ledger.bank_secret.sign(&entry);
        if bank_signature != verify_signature {
            return Err(anyhow::anyhow!("Invalid bank signature"));
        }
        let header = LedgerBrokerHeader {
            index: ledger.entries.len() as u64,
            bank_id,
            bank_signature,
        };

        let msg = LedgerMessage {
            broker_signature: ledger.broker_secret.sign(&header),
            header,
            entry,
        };
        if let Some(on_msg) = on_msg {
            on_msg(&msg);
        }
        ledger.entries.push(msg);
        Ok(())
    }
}
