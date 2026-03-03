use thiserror::Error;

pub type ParsingResult<T> = Result<T, ParsingError>;

#[derive(Debug, Error)]
pub enum ParsingError {
    #[error("{0} out of range")]
    OutOfRange(&'static str),
    #[error("mismatched version: got {got}, expected {expected}")]
    WrongVersion { got: u32, expected: u32 },
    #[error("mismatched magic: got {got:?}, expected {expected:?}")]
    WrongFourCC { got: [u8; 4], expected: [u8; 4] },
    #[error("{0} overflow")]
    NumberOverflow(&'static str),
    #[error("{0} invalid")]
    Invalid(&'static str),
}
