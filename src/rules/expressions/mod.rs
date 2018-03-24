use span::Span;
use ast::ast::Expression;
use rules::literals::literal;
use tokens;
use ast::ast::{
    Literal
};


mod conditional;

named_attr!(
#[doc="
        Recognize all kind of expressions.
    "],
pub expression<Span, Expression>,
    alt_complete!(
        //variable              => { variable_mapper }
        //| constant_access       => { constant_access_mapper }
        literal               => { literal_mapper }
        | preceded!(
            tag!(tokens::LEFT_PARENTHESIS),
            terminated!(
                first!(expression),
                first!(tag!(tokens::RIGHT_PARENTHESIS))
            )
        )
    )
);

/*#[inline]
fn variable_mapper(variable: Variable) -> Expression {
    Expression::Variable(variable)
}

#[inline]
fn constant_access_mapper(name: Name) -> Expression {
    Expression::Name(name)
}*/

#[inline]
fn literal_mapper(literal: Literal) -> Expression {
    Expression::Literal(literal)
}