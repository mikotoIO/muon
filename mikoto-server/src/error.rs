pub enum Error {
    InternalServerError,
    NotFound,
}

impl From<Error> for hyperschema::error::Error {
    fn from(_: Error) -> Self {
        hyperschema::error::Error::InternalServerError
    }
}