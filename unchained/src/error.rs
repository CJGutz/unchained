#[derive(Debug)]
pub enum Error {
    ParseTemplate,
    LoadFile,
    InvalidParams(String),
}

pub type WebResult<T> = Result<T, Error>;
