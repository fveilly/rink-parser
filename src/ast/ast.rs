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

/// A n-ary operation.
#[derive(Debug, PartialEq)]
pub enum NAryOperation<'a> {
    /// An operation with zero operator and one operand.
    Nullary(Box<Expression<'a>>),

    /// An operation with one operator and one operand: `op x`.
    Unary {
        /// The operator.
        operator: UnaryOperator,

        /// The operand (`x`).
        operand: Box<NAryOperation<'a>>
    },

    /// An operation with one operator and two operands: `x op y`.
    Binary {
        /// The operator.
        operator: BinaryOperator,

        /// The left operand (`x`).
        left_operand: Box<NAryOperation<'a>>,

        /// The right operand (`y`).
        right_operand: Box<NAryOperation<'a>>
    },
}

/// A unary operator.
#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    /// Inverts all the bits (`~x`).
    BitwiseComplement,

    /// `--x`.
    Decrement,

    /// `++x`.
    Increment,

    /// `-x`.
    Minus,

    /// `!x`.
    Negate,

    /// `+x`.
    Plus
}

/// A binary operator.
#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    /// `x & y`.
    BitwiseAnd,

    /// `x | y`.
    BitwiseOr,

    /// `x << y`.
    BitwiseShiftLeft,

    /// `x >> y`.
    BitwiseShiftRight,

    /// `x ^ y`.
    BitwiseXor,

    /// `x / y`.
    Division,

    /// `x == y`.
    Equal,

    /// `x > y`.
    GreaterThan,

    /// `x >= y`.
    GreaterThanOrEqualTo,

    /// `x < y`.
    LessThan,

    /// `x <= y`.
    LessThanOrEqualTo,

    /// `x && y`.
    LogicalAnd,

    /// `x || y`.
    LogicalOr,

    /// `x - y`.
    Subtraction,

    /// `x % y`.
    Modulo,

    /// `x * y`.
    Multiplication,

    /// `x != y`.
    NotEqual,

    /// `x + y`.
    Addition
}

/// The variable scope.
#[derive(Debug, PartialEq)]
pub enum VariableScope {
    Local,
    Global
}

/// A variable.
#[derive(Debug)]
pub struct Variable<'a> {
    pub name: Span<'a>,
    pub scope: VariableScope,
    pub constant: bool
}

impl<'a> PartialEq for Variable<'a> {
    fn eq(&self, other: &Variable<'a>) -> bool {
        self.scope == other.scope && self.name.as_slice() == other.name.as_slice()
    }
}

/// An expression.
#[derive(Debug, PartialEq)]
pub enum Expression<'a> {
    /// A literal.
    Literal(Literal<'a>),

    /// A n-ary operation.
    NAryOperation(NAryOperation<'a>),

    /// A variable.
    Variable(Variable<'a>)
}