use serde::{Serialize, Deserialize};
use crate::render::Mode;

use super::{account::AccountType, bank::Bank};

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum FocusOn {
    Username
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Deserialize, Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct LocalApp {
    pub username: String,
    pub mode: Mode,
    pub focus_on: Option<FocusOn>,
    pub bank: Option<Bank>,

    pub from_account: AccountType,
    pub to_account: AccountType,
    pub to_user: String,
    pub transfer_amount: u64,

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

            from_account: AccountType::Wallet,
            to_account: AccountType::Savings,
            to_user: Default::default(),
            transfer_amount: 0,

            dialog_visible: false,
            dialog_title: Default::default(),
            dialog_msg: Default::default()
        }
    }
}
