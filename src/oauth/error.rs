use oauth2::RequestTokenError;

#[derive(Debug, derive_more::Display)]
pub enum Error {
    #[display(fmt = "Redis error: {}", _0)]
    Redis(redis::RedisError),
    Authentication,
    BadRequest,
    /// Response Parse Failed
    Parse,
    Other(&'static str),
}

impl From<redis::RedisError> for Error {
    fn from(err: redis::RedisError) -> Self {
        Error::Redis(err)
    }
}

impl actix_web::error::ResponseError for Error {
    fn error_response(&self) -> actix_web::HttpResponse {
        match self {
            Error::Authentication => actix_web::HttpResponse::Unauthorized().finish(),
            Error::BadRequest => actix_web::HttpResponse::BadRequest().finish(),
            Error::Redis(_) | Error::Parse | Error::Other(_) => {
                log::error!("Internal server error: {:?}", self);
                actix_web::HttpResponse::InternalServerError().finish()
            }
        }
    }
}

impl<RE: std::error::Error, T: oauth2::ErrorResponse> From<RequestTokenError<RE, T>> for Error {
    fn from(err: RequestTokenError<RE, T>) -> Self {
        match err {
            RequestTokenError::Parse(err, _) => {
                log::error!("OAuth2 parse error: {:?}", err);
                Error::Parse
            }
            RequestTokenError::ServerResponse(err) => {
                log::error!("OAuth2 server response error: {:?}", err);
                Error::Authentication
            }
            _ => {
                log::error!("OAuth2 error: {:?}", err);
                Error::Other("OAuth2 error")
            }
        }
    }
}
