#[derive(Debug)]
pub enum Error {
    ParseTemplate(String),
    LoadFile(String),
    InvalidParams(String),
    Connection(String),
}

pub type WebResult<T> = Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Connection(value.to_string())
    }
}
