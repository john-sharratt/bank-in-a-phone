use std::fmt::Display;

use crate::bank::Bank;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ResponseCreateBank {
    Created { bank: Bank },
    AlreadyExists { err_msg: String },
}

impl Display for ResponseCreateBank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseCreateBank::Created { bank } => {
                write!(f, "ResponseCreateBank::Created ({})", bank.owner)
            }
            ResponseCreateBank::AlreadyExists { err_msg } => {
                write!(f, "ResposneCreateBank::AlreadyExists({})", err_msg)
            }
        }
    }
}
