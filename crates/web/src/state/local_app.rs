use std::collections::HashMap;

use crate::{render::Mode, ws::WebSocket};
use immutable_bank_model::{
    account::AccountType, bank::Bank, header::LedgerHeader, ledger::Ledger,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FocusOn {
    Username,
    Amount,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BankAndPassword {
    pub bank: Bank,
    pub password_hash: String,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Deserialize, Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct LocalApp {
    pub username: String,
    pub password: String,
    pub confirm_password: String,
    pub mode: Mode,
    pub focus_on: Option<FocusOn>,

    pub banks: HashMap<String, BankAndPassword>,

    pub session: Option<String>,
    pub ledger: Ledger,

    #[serde(skip, default)]
    pub ws: WebSocket,
    pub pending: Option<LedgerHeader>,

    #[serde(skip, default)]
    pub init: bool,

    pub from_account: AccountType,
    pub to_account: AccountType,
    pub to_user: String,
    pub transfer_amount: u64,
    pub description: String,

    // Shows a dialog box on top of all the menus
    pub dialog_visible: bool,
    // Title of the dialog box to be displayed
    pub dialog_title: String,
    // Message included in the dialog box
    pub dialog_msg: String,
}

impl LocalApp {
    pub fn bank(&self) -> Option<&Bank> {
        let session = self.session.as_ref()?;
        self.banks.get(session).map(|b| &b.bank)
    }

    pub fn bank_mut(&mut self) -> Option<&mut Bank> {
        let session = self.session.as_ref()?;
        self.banks.get_mut(session).map(|b| &mut b.bank)
    }
}

impl Default for LocalApp {
    fn default() -> Self {
        Self {
            username: Default::default(),
            password: Default::default(),
            confirm_password: Default::default(),
            mode: Mode::NewAccount,
            focus_on: Some(FocusOn::Username),

            banks: Default::default(),
            session: None,
            ledger: Ledger::default(),
            ws: WebSocket::default(),
            pending: None,

            init: false,

            from_account: AccountType::Wallet,
            to_account: AccountType::Savings,
            to_user: Default::default(),
            transfer_amount: 0,
            description: Default::default(),

            dialog_visible: false,
            dialog_title: Default::default(),
            dialog_msg: Default::default(),
        }
    }
}
