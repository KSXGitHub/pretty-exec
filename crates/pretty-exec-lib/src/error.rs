use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Program is not specified")]
    ProgramNotSpecified,
    #[error("Failed to get status code")]
    StatusCodeAcquisitionFailure,
    #[error("Failed to spawn subprocess: {}", _0)]
    ExecutionError(io::Error),
}
