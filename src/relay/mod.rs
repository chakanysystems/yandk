use crate::Error;
use crate::Result;
use ewebsock::Options;
use ewebsock::{WsReceiver, WsSender};
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
pub struct Relay {
    pub url: String,
    write: Option<WsSender>,
    recieve: Option<WsReceiver>,
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

    pub fn connect(&mut self) -> Result<()> {
        self.status = RelayStatus::Connecting;
        info!("Connecting to Relay {}", self.url);
        self.stats.add_attempt();

        let (write, recieve) = ewebsock::connect(self.url.clone(), Options::default())?;
        self.recieve.replace(recieve);
        self.write.replace(write);

        self.status = RelayStatus::Connected;
        self.stats.add_success();
        info!("Successfully connected to Relay {}", &self.url);
        Ok(())
    }

    pub fn connect_with_wakeup(
        &mut self,
        wake_up: impl Fn() + Send + Sync + 'static,
    ) -> Result<()> {
        self.status = RelayStatus::Connecting;
        info!("Connecting to Relay {}", self.url);
        self.stats.add_attempt();

        let (write, recieve) =
            ewebsock::connect_with_wakeup(self.url.clone(), Options::default(), wake_up)?;
        self.recieve.replace(recieve);
        self.write.replace(write);

        self.status = RelayStatus::Connected;
        self.stats.add_success();
        info!("Successfully connected to Relay {}", &self.url);
        Ok(())
    }

    pub fn try_recv(&mut self) -> Result<Option<ewebsock::WsEvent>> {
        if let Some(recv) = &mut self.recieve {
            return Ok(recv.try_recv());
        }

        // Since there was no reciever, there's no possible way we are connected!
        Err(Error::NotConnected)
    }

    /// todo: implement proper error handling
    pub fn send(&mut self, msg: ewebsock::WsMessage) -> Result<()> {
        if self.status != RelayStatus::Connected {
            return Err(Error::NotConnected);
        }
        if let Some(write) = &mut self.write {
            write.send(msg.clone());
        }
        Ok(())
    }
}
