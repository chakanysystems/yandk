use std::borrow::BorrowMut;

use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_tungstenite::tungstenite::connect;

use self::stats::RelayStats;
use tokio::task;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
pub mod stats;
use crate::Error;
use crate::Result;
use std::sync::{Arc, Mutex};
use tokio_tungstenite::tungstenite::Message;
use tracing::info;

#[derive(Debug)]
pub struct Relay {
    pub url: String,
    pub conn: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    pub stats: stats::RelayStats,
}

impl Relay {
    pub fn new(url: String) -> Result<Self> {
        if url.starts_with("ws://") {
            return Err(Error::InsecureConnection);
        }
        Ok(Self {
            url,
            conn: None,
            stats: RelayStats::new(),
        })
    }

    pub async fn connect(&mut self) -> Result<()> {
        info!("Connecting to {}", &self.url);
        self.stats.add_attempt();
        let options = tokio_tungstenite::tungstenite::protocol::WebSocketConfig::default();
        let (mut ws_stream, _) =
            tokio_tungstenite::connect_async_with_config(&self.url, Some(options), false).await?;
        let rr = async move {
            while let Some(message) = ws_stream.next().await {
                match message {
                    Ok(msg) => {
                        match msg {
                            Message::Text(text) => {
                                println!("{:?}", text);
                            }
                            Message::Ping(data) => {
                                println!("PING!");
                            }
                            _ => {
                                // who cares
                            }
                        }
                    }
                    Err(e) => {
                        // rip
                    }
                }
            }
        };
        self.stats.add_success();
        tokio::pin!(rr);
        tokio::select! {
            val = rr => {
                println!("completed somehow");
            }
        };
        Ok(())
    }
}
