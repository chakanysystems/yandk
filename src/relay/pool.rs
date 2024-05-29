use super::message::ClientMessage;
use super::message::RelayMessage;
use super::Relay;
use super::Subscription;
use crate::websocket;
use crate::Event;
use crate::Filter;
use crate::{Error, Result};
use std::collections::HashMap;
use tracing::{info, warn};

/// manages a group of relays.
#[derive(Debug)]
pub struct RelayPool {
    relays: HashMap<String, Relay>, // maybe we could make this &str
    subs: Vec<Subscription>,
}

#[allow(clippy::new_without_default)]
impl RelayPool {
    pub fn new() -> Self {
        info!("Initializing RelayPool");
        Self {
            relays: HashMap::new(),
            subs: Vec::new(),
        }
    }

    pub async fn add_relay(&mut self, url: String) -> Result<()> {
        if self.relays.contains_key(&url) {
            warn!("Already connected to relay {}, not connecting", &url);
            return Err(Error::AlreadyConnected);
        }
        let mut relay = Relay::new(url)?;
        relay.connect().await?;
        Ok(())
    }

    /// TODO: group subscriptions together
    pub fn add_subscription(&mut self, sub: Subscription) -> Result<()> {
        let current_subs = self.subs.clone();
        let mut cloned_sub = sub.clone();
        let cloned_filters = &cloned_sub.filters;

        // Collect the indices of the filters to be removed
        let mut indices_to_remove = Vec::new();

        for subs in current_subs.into_iter() {
            for filter in subs.filters.into_iter() {
                for (i, new_filter) in cloned_filters.iter().enumerate() {
                    if *new_filter == filter {
                        warn!("deduping filter from subscription because already present in active subscription");
                        indices_to_remove.push(i);
                    }
                }
            }
        }

        // Remove the filters by their indices, in reverse order
        indices_to_remove.sort_unstable_by(|a, b| b.cmp(a)); // Sort in descending order
        for i in indices_to_remove {
            cloned_sub.filters.remove(i);
        }
        self.subs.push(cloned_sub.clone());
        let id = sub.id.clone();
        let client_message: ClientMessage = cloned_sub.into();
        let serialized_msg = serde_json::to_string(&client_message).unwrap();
        let message_to_send = websocket::Message::Text(serialized_msg);
        let relays = &mut self.relays;
        for (url, relay) in &mut relays.into_iter() {
            info!("Adding subscription {} to {}", &id, url);
            relay.send(message_to_send.clone())?;
        }
        Ok(())
    }

    pub fn recv(&mut self) -> Result<Option<String>> {
        for relay in &mut self.relays {
            while let Some(message) = relay.1.recv()? {
                match message {
                    websocket::Message::Text(t) => {
                        return Ok(Some(t));
                    }
                    _ => {}
                }
            }
        }
        Ok(None)
    }
}
