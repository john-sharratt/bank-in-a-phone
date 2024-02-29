pub mod create;
pub mod bank_summary;
pub mod move_money;
pub mod dialog;
pub mod send_money;

#[derive(Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum Mode {
    Create,
    Summary,
    MoveMoney,
    SendMoney,
}