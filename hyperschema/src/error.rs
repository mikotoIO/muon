#[derive(Debug, Clone)]
pub enum Error {
    NotFound,
    Unauthorized,
    Forbidden,
    Timeout,
    InternalServerError,
    BadRequest,
}

impl From<rmp_serde::encode::Error> for Error {
    fn from(_: rmp_serde::encode::Error) -> Self {
        Error::BadRequest
    }
}

impl From<rmp_serde::decode::Error> for Error {
    fn from(_: rmp_serde::decode::Error) -> Self {
        Error::InternalServerError
    }
}
