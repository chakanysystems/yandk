use serde::{Deserialize, Serialize};

use crate::Pubkey;

#[derive(Debug, Deserialize, Serialize)]
pub struct Filter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<Pubkey>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinds: Option<Vec<u64>>,
}

impl Filter {
    pub fn new() -> Self {
        Self {
            ids: None,
            authors: None,
            kinds: None,
        }
    }
}
