use std::sync::mpsc::TryRecvError;

#[derive(Debug)]
pub enum Error {
    InsecureConnection,
    WsError(ewebsock::Error),
    NotConnected,
    DbError(nostrdb::Error),
    AlreadyConnected,
    HexError(hex::FromHexError),
    TryFromSliceError(std::array::TryFromSliceError),
    TryRecvError(TryRecvError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::InsecureConnection => write!(f, "connection is not wss, will not connect!"),
            Error::NotConnected => write!(f, "not connected, cannot send message."),
            Error::AlreadyConnected => write!(f, "already connected to this relay."),
            Error::WsError(ref e) => e.fmt(f),
            Error::DbError(ref e) => e.fmt(f),
            Error::HexError(ref e) => e.fmt(f),
            Error::TryFromSliceError(ref e) => e.fmt(f),
            Error::TryRecvError(ref e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::InsecureConnection => None,
            Error::WsError(ref _e) => None,
            Error::NotConnected => None,
            Error::DbError(ref e) => Some(e),
            Error::AlreadyConnected => None,
            Error::HexError(ref e) => Some(e),
            Error::TryFromSliceError(ref e) => Some(e),
            Error::TryRecvError(ref e) => Some(e),
        }
    }
}

impl From<ewebsock::Error> for Error {
    fn from(value: ewebsock::Error) -> Error {
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

impl From<TryRecvError> for Error {
    fn from(value: TryRecvError) -> Self {
        Error::TryRecvError(value)
    }
}
