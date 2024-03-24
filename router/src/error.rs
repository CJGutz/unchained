#[derive(Debug)]
pub enum Error {
    ParseTemplate,
    LoadFile,
    InvalidParams,
}

pub type WebResult<T> = Result<T, Error>;
