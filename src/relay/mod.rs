use self::stats::RelayStats;
use crate::websocket;
use crate::Error;
use crate::Result;
use futures_util::stream::SplitSink;
use futures_util::stream::SplitStream;
use futures_util::SinkExt;
use std::ops::ControlFlow;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, Mutex, RwLock};
use tokio_tungstenite::tungstenite;
use tokio_tungstenite::{connect_async_with_config, MaybeTlsStream, WebSocketStream};
use tracing::{debug, error, info, warn};

pub mod stats;

type WsConnection = WebSocketStream<MaybeTlsStream<TcpStream>>;

#[derive(Debug)]
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
    pub status: Arc<RwLock<RelayStatus>>,
    pub stats: stats::RelayStats,
}

impl Relay {
    pub fn new(url: String) -> Result<Self> {
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
        info!("Connecting to {}", &self.url);
        self.stats.add_attempt();

        let (mut write, mut recieve) = websocket::connect(self.url.clone()).await?;
        while let Some(msg) = recieve.recv().await {
            info!("recieved message from relay {}", msg);
        }

        {
            let mut status_lock = cloned_status.write().await;
            *status_lock = RelayStatus::Connected;
        }
        self.stats.add_success();
        Ok(())
    }
}
