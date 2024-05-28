pub mod event;
pub use event::Event;
#[cfg(feature = "coordinator")]
pub mod coordinator;
#[cfg(feature = "coordinator")]
pub use coordinator::Coordinator;
pub mod error;

#[cfg(feature = "relay")]
pub mod relay;

pub use error::Error;
pub(crate) mod bech32;
pub mod filter;
pub mod pubkey;
pub use filter::Filter;
pub use pubkey::Pubkey;
#[cfg(feature = "websockets")]
pub mod websocket;

pub type Result<T> = std::result::Result<T, Error>;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
