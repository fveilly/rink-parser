pub mod operations;
pub mod variables;

use span::Span;
use ast::ast::Expression;
use rules::literals::literal;
use tokens;

use self::variables::variable;

use ast::ast::{
    Literal,
    Variable
};

named_attr!(
#[doc="
        Recognize all kind of expressions.
    "],
pub expression<Span, Expression>,
    alt_complete!(
        variable              => { variable_mapper }
        | literal             => { literal_mapper }
        | preceded!(
            tag!(tokens::LEFT_PARENTHESIS),
            terminated!(
                first!(expression),
                first!(tag!(tokens::RIGHT_PARENTHESIS))
            )
        )
    )
);

#[inline]
fn variable_mapper(variable: Variable) -> Expression {
    Expression::Variable(variable)
}

#[inline]
fn literal_mapper(literal: Literal) -> Expression {
    Expression::Literal(literal)
}
