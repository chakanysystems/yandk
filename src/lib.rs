pub mod event;

pub use event::Event;
pub mod coordinator;
pub mod error;
pub mod relay;
pub mod worker;
pub use error::Error;
pub(crate) mod websocket;

pub(crate) type Result<T> = std::result::Result<T, Error>;

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
