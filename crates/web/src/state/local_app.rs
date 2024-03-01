use crate::{render::Mode, ws::WebSocket};
use immutable_bank_model::{account::AccountType, bank::Bank, ledger::Ledger};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum FocusOn {
    Username,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Deserialize, Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct LocalApp {
    pub username: String,
    pub mode: Mode,
    pub focus_on: Option<FocusOn>,

    pub bank: Option<Bank>,
    pub ledger: Ledger,

    #[serde(skip, default)]
    pub ws: WebSocket,
    pub pending: Option<u64>,

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

impl Default for LocalApp {
    fn default() -> Self {
        Self {
            username: Default::default(),
            mode: Mode::Create,
            focus_on: Some(FocusOn::Username),

            bank: None,
            ledger: Ledger::default(),
            ws: WebSocket::default(),
            pending: None,

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
