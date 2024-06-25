use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TypeError {
    #[error("Hit end of data")]
    EndOfData,
    #[error("Parsed VarInt too big")]
    VarIntTooBig,
    #[error("Parsed VarLong too big")]
    VarLongTooBig,

    #[error("{0}")]
    FromUtf8Error(#[from] FromUtf8Error),
}