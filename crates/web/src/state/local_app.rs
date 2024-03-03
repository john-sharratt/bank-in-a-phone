use std::collections::HashMap;

use crate::{render::Mode, ws::WebSocket};
use bytes::Bytes;
use immutable_bank_model::{
    account::AccountType, bank::Bank, bank_id::BankId, ledger::Ledger, password_hash::PasswordHash,
    secret::LedgerSecret,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FocusOn {
    Username,
    Password,
    Amount,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BankWithSecrets {
    pub bank_id: BankId,
    pub password: PasswordHash,
    pub secret: LedgerSecret,
}

#[derive(Debug)]
pub struct LocalSession {
    pub bank_id: BankId,
    pub ws: WebSocket,
}

impl LocalSession {
    pub fn new<ID>(bank_id: ID) -> Self
    where
        ID: Into<BankId>,
    {
        let bank_id: BankId = bank_id.into();
        Self {
            bank_id: bank_id.clone(),
            ws: WebSocket::new(bank_id),
        }
    }
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
    pub dark_mode: bool,

    pub banks: HashMap<BankId, BankWithSecrets>,

    #[serde(skip, default)]
    pub session: Option<LocalSession>,
    pub ledger: Ledger,

    #[serde(skip, default)]
    pub last_reconnects: u64,

    #[serde(skip, default)]
    pub pending: Option<std::sync::mpsc::Receiver<anyhow::Result<Bytes>>>,
    #[serde(skip, default)]
    pub pending_callback: Option<
        Box<dyn FnOnce(Bytes, &mut LocalApp, &mut eframe::Frame) -> anyhow::Result<()> + 'static>,
    >,

    #[serde(skip, default)]
    pub init1: bool,
    #[serde(skip, default)]
    pub init2: bool,

    pub from_account: AccountType,
    pub to_account: AccountType,
    pub to_bank: String,
    pub transfer_amount: u64,
    pub description: String,

    // Shows a dialog box on top of all the menus
    #[serde(skip, default)]
    pub dialog_visible: bool,
    // Title of the dialog box to be displayed
    pub dialog_title: String,
    // Message included in the dialog box
    pub dialog_msg: String,
}

impl LocalApp {
    pub fn bank_with_secrets(&self) -> Option<&BankWithSecrets> {
        let bank_id = self.session.as_ref()?.bank_id.clone();
        self.banks.get(&bank_id)
    }

    pub fn bank_with_secrets_mut(&mut self) -> Option<&mut BankWithSecrets> {
        let bank_id = self.session.as_ref()?.bank_id.clone();
        self.banks.get_mut(&bank_id)
    }

    pub fn bank(&self) -> Option<&Bank> {
        let bank_id = self.session.as_ref()?.bank_id.clone();
        self.ledger.bank(bank_id)
    }
}

impl Default for LocalApp {
    fn default() -> Self {
        Self {
            username: Default::default(),
            password: Default::default(),
            confirm_password: Default::default(),
            mode: Mode::Login,
            focus_on: Some(FocusOn::Username),
            dark_mode: true,

            banks: Default::default(),
            session: None,
            ledger: Ledger::default(),
            pending: None,
            pending_callback: None,

            init1: false,
            init2: false,
            last_reconnects: 0,

            from_account: AccountType::Wallet,
            to_account: AccountType::Savings,
            to_bank: Default::default(),
            transfer_amount: 0,
            description: Default::default(),

            dialog_visible: false,
            dialog_title: Default::default(),
            dialog_msg: Default::default(),
        }
    }
}
