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

#[derive(Debug,PartialEq,Eq,Hash,Clone)]
pub enum ErrorKindExtension {
    BinaryDigit,

    /// The datum starts as a string but is too short to be a string.
    StringTooShort,

    /// The string open character is not correct.
    StringInvalidOpeningCharacter,

    /// The string is not correctly encoded (expect UTF-8).
    StringInvalidEncoding,

    // Invalid identifier
    Identifier

}

impl ErrorKindExtension {
    pub fn description(&self) -> &str {
        match *self {
            ErrorKindExtension::BinaryDigit                         => "Binary digit",
            ErrorKindExtension::StringTooShort                      => "String too short",
            ErrorKindExtension::StringInvalidOpeningCharacter       => "String invalid opening character",
            ErrorKindExtension::StringInvalidEncoding               => "String invalid encoding",
            ErrorKindExtension::Identifier                          => "Invalid identifier"
        }
    }
}
