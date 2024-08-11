
#[derive(Debug)]
pub enum ApiError {
    TokenExpired,
}

#[derive(Debug)]
pub enum Error {
    ApiError(ApiError),
    Reqwest(reqwest::Error),
    Other(Box<dyn std::error::Error>),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Reqwest(err)
    }
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        Error::Other(err)
    }
}