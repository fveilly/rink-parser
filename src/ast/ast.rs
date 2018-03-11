use super::super::span::Span;

/// A token is a structure pairing a span to any data.
#[derive(Debug, PartialEq)]
pub struct Token<'a, T> {
    /// Value of the token.
    pub value: T,

    /// The attached span of the value.
    pub span: Span<'a>
}

impl<'a, T> Token<'a, T> {
    pub fn new(value: T, span: Span<'a>) -> Self {
        Token {
            value: value,
            span : span
        }
    }
}

/// A literal represents a fixed value, aka an atom.
#[derive(Debug, PartialEq)]
pub enum Literal<'a> {
    /// An integer (eg. a binary, octal, decimal or hexadecimal number).
    Integer(Token<'a, i64>),

    /// A real (eg. an exponential number).
    Real(Token<'a, f64>),

    /// A boolean.
    Boolean(Token<'a, bool>),

    /// A string.
    String(Token<'a, String>)
}