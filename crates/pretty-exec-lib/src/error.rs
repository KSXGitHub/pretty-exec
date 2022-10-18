use derive_more::From;
use std::io;
use thiserror::Error;

#[derive(Debug, From, Error)]
pub enum Error {
    #[error("Program is not specified")]
    MissingProgram,
    #[error("Failed to get status code")]
    StatusCodeAcquisitionFailure,
    #[error("{}", _0)]
    IoError(io::Error),
}
