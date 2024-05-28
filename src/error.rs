use core::fmt;
use tokio_tungstenite::tungstenite;

#[derive(Debug)]
pub enum Error {
    InsecureConnection,
    WsError(tungstenite::Error),
    NotConnected,
    DbError(nostrdb::Error),
    AlreadyConnected,
    HexError(hex::FromHexError),
    TryFromSliceError(std::array::TryFromSliceError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InsecureConnection => write!(f, "connection is not wss, will not connect!"),
            Error::NotConnected => write!(f, "not connected, cannot send message."),
            Error::AlreadyConnected => write!(f, "already connected to this relay."),
            Error::WsError(ref e) => e.fmt(f),
            Error::DbError(ref e) => e.fmt(f),
            Error::HexError(ref e) => e.fmt(f),
            Error::TryFromSliceError(ref e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::InsecureConnection => None,
            Error::WsError(ref e) => Some(e),
            Error::NotConnected => None,
            Error::DbError(ref e) => Some(e),
            Error::AlreadyConnected => None,
            Error::HexError(ref e) => Some(e),
            Error::TryFromSliceError(ref e) => Some(e),
        }
    }
}

impl From<tungstenite::Error> for Error {
    fn from(value: tungstenite::Error) -> Error {
        Error::WsError(value)
    }
}

impl From<nostrdb::Error> for Error {
    fn from(value: nostrdb::Error) -> Self {
        Error::DbError(value)
    }
}

impl From<hex::FromHexError> for Error {
    fn from(value: hex::FromHexError) -> Self {
        Error::HexError(value)
    }
}

impl From<std::array::TryFromSliceError> for Error {
    fn from(value: std::array::TryFromSliceError) -> Self {
        Error::TryFromSliceError(value)
    }
}
