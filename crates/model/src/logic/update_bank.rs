use crate::{
    ledger::{Ledger, LedgerMessage},
    ledger_type::LedgerEntry,
    requests::update_bank::RequestUpdateBank,
    responses::update_bank::ResponseUpdatedBank,
};

impl Ledger {
    pub fn update_bank(
        &mut self,
        req: RequestUpdateBank,
        mut on_msg: impl FnMut(&LedgerMessage),
    ) -> anyhow::Result<ResponseUpdatedBank> {
        // The total amount of money in the bank can not change from the last update
        // otherwise this has allowed the requester to create or destroy money
        let bank = match self.ledger_for(req.bank.owner.clone()) {
            Some(b) => b,
            None => {
                return Ok(ResponseUpdatedBank::InvalidBank {
                    bank_id: req.bank.owner.clone(),
                })
            }
        };
        let total_before = bank.bank().map(|b| b.total_funds()).unwrap_or_default();
        let total_after = req.bank.total_funds();
        if total_after != total_before {
            return Ok(ResponseUpdatedBank::InvalidUpdate { err_msg: format!("Money has either been created or destroy, this is not allowed, instead transfer the money.") });
        }

        // Add the entry to the ledger
        self.add(
            req.bank.owner.clone(),
            LedgerEntry::UpdateBank(req.bank.clone()),
            req.signature,
            Some(&mut on_msg),
        )?;

        Ok(ResponseUpdatedBank::Updated { bank: req.bank })
    }
}
