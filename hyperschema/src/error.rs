#[derive(Debug, Clone)]
pub enum Error {
    DeserializationFailed,
    SerializationFailed,
}

impl From<rmp_serde::encode::Error> for Error {
    fn from(_: rmp_serde::encode::Error) -> Self {
        Error::SerializationFailed
    }
}
