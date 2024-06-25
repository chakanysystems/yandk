use super::message::ClientMessage;
use super::Relay;
use super::Subscription;
use crate::{Error, Result};
use std::collections::HashMap;
use tracing::{error, info, warn};

/// manages a group of relays.
pub struct RelayPool {
    relays: HashMap<String, Relay>,
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
        let message_to_send = ewebsock::WsMessage::Text(serialized_msg);
        let relays = &mut self.relays;
        for (url, relay) in &mut relays.into_iter() {
            info!("Adding subscription {} to {}", &id, url);
            relay.send(message_to_send.clone())?;
        }
        Ok(())
    }

    pub fn add_relay(&mut self, url: String) -> Result<()> {
        let cloned_url = url.clone();
        if self.relays.contains_key(&cloned_url) {
            error!(
                "already connected to {}, not opening another connection",
                cloned_url
            );
            return Err(Error::AlreadyConnected);
        }
        let mut new_relay = crate::relay::Relay::new(cloned_url)?;
        new_relay.connect()?;
        Ok(())
    }

    pub fn add_relay_with_wakeup(
        &mut self,
        url: String,
        wake_up: impl Fn() + Send + Sync + 'static,
    ) -> Result<()> {
        let cloned_url = url.clone();
        if self.relays.contains_key(&cloned_url) {
            error!(
                "already connected to {}, not opening another connection",
                cloned_url
            );
            return Err(Error::AlreadyConnected);
        }
        let mut new_relay = crate::relay::Relay::new(cloned_url)?;
        new_relay.connect_with_wakeup(wake_up)?;
        Ok(())
    }

    pub fn keepalive(&mut self) {}

    pub fn try_recv(&mut self) -> Result<Option<String>> {
        for relay in &mut self.relays {
            if let Ok(result) = relay.1.try_recv() {
                if let Some(event) = result {
                    match event {
                        ewebsock::WsEvent::Message(msg) => match msg {
                            ewebsock::WsMessage::Text(txt) => return Ok(Some(txt)),
                            _ => {} //  TODO: FIX
                        },
                        _ => {}
                    }
                }
            }
        }
        Ok(None)
    }
}
