pub mod bank_summary;
pub mod create;
pub mod dialog;
pub mod login;
pub mod move_money;
pub mod send_money;
pub mod pending;

#[derive(
    Clone, Copy, serde::Deserialize, serde::Serialize, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub enum Mode {
    NewAccount,
    Login,
    Summary,
    MoveMoney,
    SendMoney,
}
