/// Hold the result of a parser.
pub use nom::IResult as Result;

/// Contain the error that a parser_old can return.
pub use nom::Err as Error;

/// Indicate which parser_old has returned an error.
pub use nom::ErrorKind;

/// Indicate the context of an error.
pub use nom::Context;

/// Contain information on needed data if a parser_old returned `Incomplete`.
pub use nom::Needed;

use std::num;

#[derive(Debug,PartialEq,Eq,Hash,Clone)]
pub enum ErrorKindExtension {
    BinaryDigit
}

pub fn error_to_u32(e: &ErrorKindExtension) -> u32 {
    match *e {
        ErrorKindExtension::BinaryDigit                 => 0
    }
}

impl ErrorKindExtension {
    pub fn description(&self) -> &str {
        match *self {
            ErrorKindExtension::BinaryDigit             => "Hexadecimal Digit"
        }

    }
}

impl From<ErrorKindExtension> for u32 {
    fn from(e: ErrorKindExtension) -> Self {
        match e {
            ErrorKindExtension::BinaryDigit             => 0
        }
    }
}
