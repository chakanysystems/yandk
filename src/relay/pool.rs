use nostrdb::Filter;

use super::Relay;
use crate::Filter;
use crate::{Error, Result};
use std::collections::HashMap;
use tracing::{info, warn};

/// manages a group of relays.
#[derive(Debug)]
pub struct RelayPool {
    relays: HashMap<&'static str, Relay>, // maybe we could make this &str
    subs: HashMap<&'static str, Vec<Filter>>,
}

impl RelayPool {
    pub fn new() -> Self {
        info!("Initializing RelayPool");
        Self {
            relays: HashMap::new(),
            subs: HashMap::new(),
        }
    }

    pub async fn add_relay(&mut self, url: &'static str) -> Result<()> {
        if self.relays.contains_key(&url) {
            warn!("Already connected to relay {}, not connecting", &url);
            return Err(Error::AlreadyConnected);
        }
        let mut relay = Relay::new(url)?;
        relay.connect().await?;
        Ok(())
    }

    pub async fn add_subscription(&mut self, filters: Vec<Filter>) -> Result<()> {
        info!("Adding subscription {:?}", filters);
        Ok(())
    }
}
