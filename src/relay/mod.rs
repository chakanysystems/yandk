use crate::websocket;
use crate::Error;
use crate::Result;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex, RwLock};
use tracing::{debug, error, info, warn};

#[cfg(feature = "relay-pool")]
pub mod pool;
#[cfg(feature = "relay-pool")]
pub use pool::RelayPool;
mod stats;
pub use stats::RelayStats;

#[derive(Debug)]
pub enum RelayStatus {
    Connected,
    Connecting,
    Disconnected,
}

/// a relay
#[derive(Debug)]
pub struct Relay {
    pub url: &'static str,
    write: Option<websocket::WsSender>,
    recieve: Option<websocket::WsReciever>,
    pub status: Arc<RwLock<RelayStatus>>,
    pub stats: stats::RelayStats,
}

impl Relay {
    pub fn new(url: &'static str) -> Result<Self> {
        info!("Initializing Relay {}", url);
        if url.starts_with("ws://") {
            // this is probably bad.
            return Err(Error::InsecureConnection);
        }
        Ok(Self {
            url,
            write: None,
            recieve: None,
            status: Arc::new(RwLock::new(RelayStatus::Disconnected)),
            stats: RelayStats::new(),
        })
    }

    pub async fn connect(&mut self) -> Result<()> {
        let cloned_status = self.status.clone();
        {
            let mut status_lock = cloned_status.write().await;
            *status_lock = RelayStatus::Connecting;
        }
        info!("Connecting to Relay {}", &self.url);
        self.stats.add_attempt();

        let (mut write, mut recieve) = websocket::connect(&self.url).await?;
        while let Some(msg) = recieve.recv().await {
            info!("recieved message from relay {}", msg);
        }

        {
            let mut status_lock = cloned_status.write().await;
            *status_lock = RelayStatus::Connected;
        }
        self.stats.add_success();
        info!("Successfully connected to Relay {}", &self.url);
        Ok(())
    }
}
