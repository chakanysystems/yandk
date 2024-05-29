use crate::websocket;
use crate::Error;
use crate::Result;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tracing::{error, info};

#[cfg(feature = "relay-pool")]
pub mod pool;
#[cfg(feature = "relay-pool")]
pub use pool::RelayPool;
pub mod stats;
pub use stats::RelayStats;
pub mod message;
pub mod subscription;
pub use subscription::Subscription;

use self::message::RelayMessage;

#[derive(Debug, PartialEq, Eq)]
pub enum RelayStatus {
    Connected,
    Connecting,
    Disconnected,
}

/// a relay
#[derive(Debug)]
pub struct Relay {
    pub url: String,
    write: Option<websocket::WsSender>,
    recieve: Option<websocket::WsReciever>,
    pub status: RelayStatus,
    pub stats: stats::RelayStats,
}

impl Relay {
    pub fn new(url: String) -> Result<Self> {
        info!("Initializing Relay {}", url);
        if url.starts_with("ws://") {
            // this is probably bad.
            return Err(Error::InsecureConnection);
        }
        Ok(Self {
            url,
            write: None,
            recieve: None,
            status: RelayStatus::Disconnected,
            stats: RelayStats::new(),
        })
    }

    pub async fn connect(&mut self) -> Result<()> {
        self.status = RelayStatus::Connecting;
        info!("Connecting to Relay {}", self.url);
        self.stats.add_attempt();

        let (write, recieve) = websocket::connect(self.url.clone()).await?;
        self.recieve.replace(recieve);
        self.write.replace(write);

        self.status = RelayStatus::Connected;
        self.stats.add_success();
        info!("Successfully connected to Relay {}", &self.url);
        Ok(())
    }

    pub fn recv(&mut self) -> Result<Option<websocket::Message>> {
        if let Some(recv) = &mut self.recieve {
            while let Ok(msg) = recv.recv() {
                return Ok(Some(msg));
            }
        } else {
            return Err(Error::NotConnected);
        }
        Ok(None)
    }

    /// todo: implement proper error handling
    pub fn send(&mut self, msg: websocket::Message) -> Result<()> {
        if self.status != RelayStatus::Connected {
            return Err(Error::NotConnected);
        }
        if let Some(write) = &mut self.write {
            match write.send(msg.clone()) {
                Ok(w) => w,
                Err(e) => error!("unable to send: {}", e),
            };
        }
        Ok(())
    }
}
