use std::{error::Error, fmt::Display, num::ParseIntError};

#[derive(Clone, Debug)]
pub enum ParseEntryError {
    Format(&'static str),
    Int(ParseIntError),
    Timestamp(chrono::ParseError),
    Version(ParseVersionError),
}

impl From<chrono::ParseError> for ParseEntryError {
    fn from(e: chrono::ParseError) -> Self {
        ParseEntryError::Timestamp(e)
    }
}

impl From<ParseIntError> for ParseEntryError {
    fn from(e: ParseIntError) -> Self {
        ParseEntryError::Int(e)
    }
}

impl From<ParseVersionError> for ParseEntryError {
    fn from(e: ParseVersionError) -> Self {
        ParseEntryError::Version(e)
    }
}

#[derive(Clone, Debug)]
pub enum ParseVersionError {
    Format(&'static str),
    Int(ParseIntError),
}

impl Display for ParseVersionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseVersionError::Format(msg) => write!(f, "missing segment: {}", msg),
            ParseVersionError::Int(e) => e.fmt(f),
        }
    }
}

impl Error for ParseVersionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseVersionError::Int(e) => Some(e),
            _ => None,
        }
    }
}

impl From<ParseIntError> for ParseVersionError {
    fn from(e: ParseIntError) -> Self {
        ParseVersionError::Int(e)
    }
}
