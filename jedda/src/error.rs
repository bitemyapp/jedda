#[derive(Debug)]
pub enum Error {
    ConnectionNegotiation(String),
}

pub type Result<V> = core::result::Result<V, Error>;
