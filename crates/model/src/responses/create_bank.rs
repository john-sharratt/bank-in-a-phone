use crate::bank::Bank;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ResponseCreateBank {
    Created { bank: Bank },
    AlreadyExists { err_msg: String },
}
