use core::fmt;
use tokio_tungstenite::tungstenite;

#[derive(Debug)]
pub enum Error {
    InsecureConnection,
    WsError(tungstenite::Error),
    NotConnected,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InsecureConnection => write!(f, "connection is not wss, will not connect!"),
            Error::NotConnected => write!(f, "not connected, cannot send message."),
            Error::WsError(ref e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::InsecureConnection => None,
            Error::WsError(ref e) => Some(e),
            Error::NotConnected => None,
        }
    }
}

impl From<tungstenite::Error> for Error {
    fn from(value: tungstenite::Error) -> Error {
        Error::WsError(value)
    }
}
