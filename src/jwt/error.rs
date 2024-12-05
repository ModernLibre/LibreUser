pub enum Error {
    BadInitialize,
    InvalidToken,
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(_err: jsonwebtoken::errors::Error) -> Error {
        Error::InvalidToken
    }
}

impl From<rsa::pkcs1::Error> for Error {
    fn from(_err: rsa::pkcs1::Error) -> Error {
        Error::InvalidToken
    }
}
