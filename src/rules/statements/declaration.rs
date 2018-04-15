use span::Span;
use tokens;

use ast::ast::Statement;
use rules::expressions::variables::variable;
use rules::expressions::operations::operation;

named_attr!(
    #[doc="
        Recognize a declaration statement.
    "],
    pub declaration<Span, Statement>,
    preceded!(
        tag!(tokens::STATEMENT),
        do_parse!(
            variable: first!(variable) >>
            first!(tag!(tokens::ASSIGN)) >>
            expression: first!(operation) >>
            (Statement::Declaration(variable, expression))
        )
    )
);

#[cfg(test)]
mod tests {
    use super::declaration;
    use span::Span;

    use ast::ast::{
        Token,
        Statement,
        Variable,
        Expression,
        Literal,
        NAryOperation,
        BinaryOperator
    };

    macro_rules! nullary_operation {
        ($expression:expr) => (
            NAryOperation::Nullary(Box::new($expression))
        )
    }

    macro_rules! binary_operation {
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
    fn case_declaration_literal_boolean() {
        let input  = Span::new("~ knows_about_wager = true\n");
        let output = Ok((
            Span::new_at("\n", 26, 1, 27),
            Statement::Declaration (
                Variable (Span::new_at("knows_about_wager", 2, 1, 3)),
                Expression::NAryOperation(nullary_operation!(boolean!(true, Span::new_at("true", 22, 1, 23))))
            )
        ));

        assert_eq!(declaration(input), output);
    }

    #[test]
    fn case_declaration_expression() {
        let input  = Span::new("~ y = 2 * x * y\n");
        let output = Ok((
            Span::new_at("\n", 15, 1, 16),
            Statement::Declaration (
                Variable (Span::new_at("y", 2, 1, 3)),
                Expression::NAryOperation(
                    binary_operation!(
                        Multiplication,
                        binary_operation!(
                            Multiplication,
                            nullary_operation!(integer!(2, Span::new_at("2", 6, 1, 7))),
                            nullary_operation!(variable!(Span::new_at("x", 10, 1, 11)))
                        ),
                        nullary_operation!(variable!(Span::new_at("y", 14, 1, 15)))
                    )
                )
            )
        ));

        assert_eq!(declaration(input), output);
    }

    #[test]
    fn case_declaration_expression_with_whitespaces() {
        let input  = Span::new("~   y   =   2   *   x   *   y\n");
        let output = Ok((
            Span::new_at("\n", 29, 1, 30),
            Statement::Declaration (
                Variable (Span::new_at("y", 4, 1, 5)),
                Expression::NAryOperation(
                    binary_operation!(
                        Multiplication,
                        binary_operation!(
                            Multiplication,
                            nullary_operation!(integer!(2, Span::new_at("2", 12, 1, 13))),
                            nullary_operation!(variable!(Span::new_at("x", 20, 1, 21)))
                        ),
                        nullary_operation!(variable!(Span::new_at("y", 28, 1, 29)))
                    )
                )
            )
        ));

        assert_eq!(declaration(input), output);
    }

    #[test]
    fn case_declaration_expression_parenthesis() {
        let input  = Span::new("~ x = (x * x) - (y * y) + c\n");
        let output = Ok((
            Span::new_at("\n", 27, 1, 28),
            Statement::Declaration (
                Variable (Span::new_at("x", 2, 1, 3)),
                Expression::NAryOperation(
                    binary_operation!(
                        Addition,
                        binary_operation!(
                            Subtraction,
                            binary_operation!(
                                Multiplication,
                                nullary_operation!(variable!(Span::new_at("x", 7, 1, 8))),
                                nullary_operation!(variable!(Span::new_at("x", 11, 1, 12)))
                            ),
                            binary_operation!(
                                Multiplication,
                                nullary_operation!(variable!(Span::new_at("y", 17, 1, 18))),
                                nullary_operation!(variable!(Span::new_at("y", 21, 1, 22)))
                            )
                        ),
                        nullary_operation!(variable!(Span::new_at("c", 26, 1, 27)))
                    )
                )
            )
        ));

        assert_eq!(declaration(input), output);
    }

    #[test]
    fn case_declaration_expression_division_real() {
        let input  = Span::new("~ z = 1.2 / 0.5\n");
        let output = Ok((
            Span::new_at("\n", 15, 1, 16),
            Statement::Declaration (
                Variable (Span::new_at("z", 2, 1, 3)),
                Expression::NAryOperation(
                    binary_operation!(
                        Division,
                        nullary_operation!(real!(1.2, Span::new_at("1.2", 6, 1, 7))),
                        nullary_operation!(real!(0.5, Span::new_at("0.5", 12, 1, 13)))
                    )
                )
            )
        ));

        assert_eq!(declaration(input), output);
    }
}