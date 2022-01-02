use tracing::error;

#[derive(Debug)]
pub enum Error {
    ConnectionNegotiation(String),
}

pub type Result<V> = core::result::Result<V, Error>;

pub trait IntoJeddaError<T> {
    fn jedda_error(
        self,
        error_val: Error,
    ) -> Result<T>;

    fn negotiation_error<S>(self, message: S) -> Result<T>
        where
            S: AsRef<str>,
            Self: std::marker::Sized,
    {
        self.jedda_error(Error::ConnectionNegotiation(message.to_string()))
    }
}

impl<T, E: std::fmt::Debug> IntoJeddaError<T> for core::result::Result<T, E> {
    fn jedda_error(
        self,
        error_val: Error
    ) -> core::result::Result<T, actix_web::Error> {
        match self {
            Ok(val) => Ok(val),
            Err(err) => {
                error!("http_error: {:?}", err);
                Err(error_val)
            }
        }
    }
}
