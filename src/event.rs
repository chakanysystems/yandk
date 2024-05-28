use serde::{Deserialize, Serialize};
use std::default;

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize, Clone)]
pub struct Event {
    pub id: Option<String>,
    pub sig: Option<String>,
    pub tags: Vec<Vec<String>>,
    pub pubkey: Option<String>,
    pub kind: u32,
    pub created_at: i32,
    pub content: String,
}

impl default::Default for Event {
    fn default() -> Self {
        Self {
            id: None,
            sig: None,
            pubkey: None,
            tags: Vec::new(),
            created_at: 0, // lol
            kind: 1,
            content: "".to_string(),
        }
    }
}

impl Event {
    pub fn sign(&mut self) {
        unimplemented!()
    }

    pub fn hash(&mut self) {
        unimplemented!()
    }
}
