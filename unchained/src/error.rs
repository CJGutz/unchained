#[derive(Debug)]
pub enum Error {
    ParseTemplate(String),
    LoadFile(String),
    InvalidParams(String),
    Connection(String),
}

pub type WebResult<T> = Result<T, Error>;
