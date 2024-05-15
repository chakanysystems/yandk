// stripped down ewebsock

#[derive(Debug, Clone)]
pub enum WsMessage {
    Binary(Vec<u8>),
    Text(String),
    Unknown(String),
    Ping(Vec<u8>),
    Pong(Vec<u8>),
}

#[derive(Debug, Clone)]
pub enum WsEvent {
    Opened,
    Message(WsMessage),
    Error(String),
    Closed,
}

pub struct WsSender
