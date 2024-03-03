use crate::{
    ledger::{Ledger, LedgerBrokerHeader, LedgerMessage},
    ledger_type::LedgerEntry,
    requests::transfer::RequestTransfer,
    responses::transfer::ResponseTransfer,
};

impl Ledger {
    pub fn transfer(
        &mut self,
        req: RequestTransfer,
        mut on_msg: impl FnMut(&LedgerMessage),
    ) -> anyhow::Result<ResponseTransfer> {
        // Grab references to the banks we are transferring money between
        let from_bank = match self.ledger_for(req.trans.from.bank.clone()) {
            Some(b) => b,
            None => {
                log::info!("Transfer-From-Invalid: {:?}", req.trans.from.bank);
                return Ok(ResponseTransfer::InvalidBank {
                    bank_id: req.trans.from.bank.clone(),
                });
            }
        };
        let to_bank = match self.ledger_for(req.trans.to.bank.clone()) {
            Some(b) => b,
            None => {
                log::info!("Transfer-To-Invalid: {:?}", req.trans.to.bank);
                return Ok(ResponseTransfer::InvalidBank {
                    bank_id: req.trans.to.bank.clone(),
                });
            }
        };

        // Check the signature on the request using the from bank account
        let signature = from_bank.bank_secret.sign(&req.trans);
        if signature != req.signature {
            log::info!(
                "Transfer-Invalid-Signature: expected={}, actual={}",
                signature,
                req.signature
            );
            return Ok(ResponseTransfer::InvalidSignature);
        }

        // Get the accounts
        let from_acc = match from_bank.account(req.trans.from.account) {
            Some(b) => b,
            None => {
                log::info!(
                    "Transfer-Invalid-From-Account: {:?}",
                    req.trans.from.account
                );
                return Ok(ResponseTransfer::InvalidAccount {
                    bank_id: req.trans.from.bank.clone(),
                    account: req.trans.from.account,
                });
            }
        };
        if to_bank.account(req.trans.to.account).is_none() {
            log::info!("Transfer-Invalid-To-Account: {:?}", req.trans.to.account);
            return Ok(ResponseTransfer::InvalidAccount {
                bank_id: req.trans.to.bank.clone(),
                account: req.trans.to.account,
            });
        };

        // Check the source bank account has enough funds
        if req.trans.amount_cents > from_acc.balance_cents {
            log::info!("Transfer-Insufficient-Funds: {:?}", req.trans);
            return Ok(ResponseTransfer::InsufficientFunds {
                requested: req.trans.amount_cents,
                available: from_acc.balance_cents,
                bank_id: req.trans.to.bank.clone(),
                account: req.trans.to.account,
            });
        }

        // We renew the reference as it may be the same bank
        let ledger = self.ledger_for(req.trans.from.bank.clone()).unwrap();
        let mut new_bank = ledger.bank().unwrap().clone();

        // First we attempt to add an entry for the source bank account
        new_bank
            .account_mut(req.trans.from.account)
            .unwrap()
            .balance_cents -= req.trans.amount_cents;
        let new_entry = LedgerEntry::UpdateBank(new_bank.clone());
        let signature = self
            .banks
            .get(&req.trans.from.bank)
            .unwrap()
            .bank_secret
            .sign(&new_entry);
        self.add(
            new_bank.owner.clone(),
            new_entry,
            signature,
            Some(&mut on_msg),
        )?;

        // We renew the reference as it may be the same bank
        let ledger = self.ledger_for(req.trans.to.bank.clone()).unwrap();
        let mut new_bank = ledger.bank().unwrap().clone();

        // Now add it to the destination bank account
        new_bank
            .account_mut(req.trans.to.account)
            .unwrap()
            .balance_cents += req.trans.amount_cents;
        let new_entry = LedgerEntry::UpdateBank(new_bank.clone());
        let signature = self
            .banks
            .get(&req.trans.to.bank)
            .unwrap()
            .bank_secret
            .sign(&new_entry);
        self.add(
            new_bank.owner.clone(),
            new_entry,
            signature,
            Some(&mut on_msg),
        )?;

        // Now add transactions into the history
        let ledger = self.ledger_mut_for(req.trans.from.bank.clone()).unwrap();
        let entry = LedgerEntry::Transaction {
            transaction: req.trans.clone(),
        };
        let header = LedgerBrokerHeader {
            index: ledger.entries.len() as u64,
            bank_id: req.trans.from.bank.clone(),
            bank_signature: ledger.bank_secret.sign(&entry),
        };
        let msg = LedgerMessage {
            broker_signature: ledger.broker_secret.sign(&entry),
            header,
            entry,
        };
        on_msg(&msg);
        ledger.entries.push(msg);

        // If they are different banks
        if req.trans.from.bank != req.trans.to.bank {
            let ledger = self.ledger_mut_for(req.trans.to.bank.clone()).unwrap();
            let entry = LedgerEntry::Transaction {
                transaction: req.trans.clone(),
            };
            let header = LedgerBrokerHeader {
                index: ledger.entries.len() as u64,
                bank_id: req.trans.to.bank.clone(),
                bank_signature: ledger.bank_secret.sign(&entry),
            };
            let msg = LedgerMessage {
                broker_signature: ledger.broker_secret.sign(&entry),
                header,
                entry,
            };
            on_msg(&msg);
            ledger.entries.push(msg);
        }

        // There, the money is transferred!
        Ok(ResponseTransfer::Transferred)
    }
}
