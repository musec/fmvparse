/*
 * © 2021 Arastoo Bozorgi
 * All rights reserved.
 */

use std::fmt;
use std::io::ErrorKind;
use std::str::Utf8Error;

///
/// Problems that can arise in UPSS.
///
#[derive(Debug)]
pub enum Error {

    /// There was an error reading formatted data
    FormatError {
        format_of: String,
        detail: String,
    },

    /// Configuration information passed to UPSS was incorrect
    InvalidConfig(String),

    /// There has been an attempt to access data at an invalid index
    InvalidIndex {
        kind: String,
        index: usize,
        max: usize,
    },

    /// An error occurred on the disk or network
    IO(std::io::Error),

    Unknown(String),

    /// The user (not the developer or the system) has made a mistake
    UserError(String),

    StringConversionError(String),
}

impl Error {
    pub fn config<T>(detail: T) -> Error
    where
        T: std::error::Error,
    {
        Error::InvalidConfig(format!["{:?}", detail])
    }

    pub fn format<S1, S2>(format_of: S1, detail: S2) -> Error
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Error::FormatError {
            format_of: format_of.into(),
            detail: detail.into(),
        }
    }

    pub fn index<S>(kind: S, index: usize, max: usize) -> Error
    where
        S: Into<String>,
    {
        Error::InvalidIndex {
            kind: kind.into(),
            index,
            max,
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IO(ref e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<Error> for std::io::Error {
    fn from(err: Error) -> std::io::Error {
        std::io::Error::new(ErrorKind::Other, err)
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Self {
        Error::StringConversionError(err.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::FormatError {
                ref format_of,
                ref detail,
            } => write!(f, "Format error in {}: {:?}", format_of, detail),
            Error::IO(ref err) => write!(f, "IO error: {}", err),
            Error::InvalidConfig(ref detail) => write!(f, "Configuration error: {}", detail),
            Error::InvalidIndex {
                ref kind,
                index,
                max,
            } => write!(f, "Invalid {} index: {} (max: {})", kind, index, max),
            Error::Unknown(ref detail) => write!(f, "Unknown error: {}", detail),
            Error::UserError(ref detail) => write!(f, "User error: {}", detail),
            Error::StringConversionError(ref detail) => {
                write!(f, "String conversion error: {}", detail)
            }
        }
    }
}
