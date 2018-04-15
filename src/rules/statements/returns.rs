use span::Span;
use tokens;

use ast::ast::Expression;
use ast::ast::Statement;
use rules::expressions::operations::operation;

named_attr!(
    #[doc="
        Recognize a return statement.
    "],
    pub return_statement<Span, Statement>,
    map_res!(
        preceded!(
            tag!(tokens::STATEMENT),
            preceded!(
                first!(tag!(tokens::RETURN)),
                first!(operation)
            )
        ),
        return_statement_mapper
    )
);

#[inline]
fn return_statement_mapper<'a>(expression: Expression<'a>) -> Result<Statement<'a>, ()> {
    Ok(Statement::Return(expression))
}

#[cfg(test)]
mod tests {
    use super::return_statement;
    use span::Span;

    use ast::ast::{
        Token,
        Variable,
        Expression,
        Statement,
        Literal,
        NAryOperation,
        BinaryOperator
    };

    macro_rules! nullary {
        ($expression:expr) => (
            NAryOperation::Nullary(Box::new($expression))
        )
    }

    macro_rules! binary {
        ($operator:ident, $left_operand:expr, $right_operand:expr) => (
            NAryOperation::Binary {
                operator     : BinaryOperator::$operator,
                left_operand : Box::new($left_operand),
                right_operand: Box::new($right_operand)
            }
        )
    }

    macro_rules! integer {
        ($value:expr, $span:expr) => (
            Expression::Literal(Literal::Integer(Token::new($value, $span)))
        )
    }

    macro_rules! real {
        ($value:expr, $span:expr) => (
            Expression::Literal(Literal::Real(Token::new($value, $span)))
        )
    }

    macro_rules! boolean {
        ($value:expr, $span:expr) => (
            Expression::Literal(Literal::Boolean(Token::new($value, $span)))
        )
    }

    macro_rules! variable {
        ($name:expr) => (
            Expression::Variable(Variable($name))
        )
    }

    #[test]
    fn case_return_literal_boolean() {
        let input = Span::new("~ return true\n");
        let output = Ok((
            Span::new_at("\n", 13, 1, 14),
            Statement::Return(Expression::NAryOperation(nullary!(boolean!(true, Span::new_at("true", 9, 1, 10)))))
        ));

        assert_eq!(return_statement(input), output);
    }

    #[test]
    fn case_return_expression() {
        let input = Span::new("~ return ((b - a) * k) + a\n");
        let output = Ok((
            Span::new_at("\n", 26, 1, 27),
            Statement::Return(Expression::NAryOperation(
                binary!(
                    Addition,
                    binary!(
                        Multiplication,
                        binary!(
                            Subtraction,
                            nullary!(variable!(Span::new_at("b", 11, 1, 12))),
                            nullary!(variable!(Span::new_at("a", 15, 1, 16)))
                        ),
                        nullary!(variable!(Span::new_at("k", 20, 1, 21)))
                    ),
                    nullary!(variable!(Span::new_at("a", 25, 1, 26)))
                )
            ))
        ));

        assert_eq!(return_statement(input), output);
    }

    // TODO: ~ fear++
    // TODO: ~ x = lerp(2, 8, 0.3)
    // TODO: ~ return x * exp(x, e - 1)
}