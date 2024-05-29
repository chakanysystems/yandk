use crate::Result;
use futures_util::SinkExt;
use tokio::sync::mpsc::{
    error::TryRecvError, unbounded_channel, UnboundedReceiver, UnboundedSender,
};
pub use tokio_tungstenite::tungstenite::Message;
use tracing::{error, info};

#[derive(Debug)]
pub struct WsReciever {
    pub rx: UnboundedReceiver<Message>,
}

impl WsReciever {
    pub fn recv(&mut self) -> Result<Message> {
        let recv = match self.rx.try_recv() {
            Ok(e) => e,
            Err(e) => match e {
                TryRecvError::Disconnected => return Err(e.into()),
                TryRecvError::Empty => {}
            },
        };
        Ok(recv)
    }
}

#[derive(Debug)]
pub struct WsSender {
    wr: UnboundedSender<Message>,
}

impl WsSender {
    pub fn send(
        &mut self,
        msg: Message,
    ) -> std::result::Result<(), tokio::sync::mpsc::error::SendError<Message>> {
        self.wr.send(msg)
    }
}

pub async fn connect(url: String) -> Result<(WsSender, WsReciever)> {
    let (ws_stream, _r) = tokio_tungstenite::connect_async(url).await?;
    use futures_util::StreamExt as _;
    let (mut write, mut read) = ws_stream.split();
    let (reader_wr, reader_rx): (UnboundedSender<Message>, UnboundedReceiver<Message>) =
        unbounded_channel();
    info!("trying to connect...");
    tokio::spawn(async move {
        while let Some(event) = read.next().await {
            match event {
                Ok(message) => {
                    //f
                    match message {
                        Message::Frame(_f) => error!("Recieved a frame, we do not implement this."),
                        _ => match reader_wr.send(message) {
                            Ok(e) => e,
                            Err(e) => {
                                panic!("error trying to forward message: {}", e)
                            }
                        },
                    }
                }
                Err(e) => {
                    panic!("error from websocket recieve {}", e);
                }
            }
        }
    });
    let (writer_wr, mut writer_rx): (UnboundedSender<Message>, UnboundedReceiver<Message>) =
        unbounded_channel();

    tokio::spawn(async move {
        while let Some(message) = writer_rx.recv().await {
            match write.send(message).await {
                Ok(s) => s,
                Err(e) => error!("could not send message to relay {}", e),
            };
        }
    });

    let reciever_struct = WsReciever { rx: reader_rx };
    let writer_struct = WsSender { wr: writer_wr };

    Ok((writer_struct, reciever_struct))
}
