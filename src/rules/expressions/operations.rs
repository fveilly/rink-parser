use span::Span;
use tokens;
use ast::ast::{
    NAryOperation,
    BinaryOperator,
    Expression
};
use super::expression;

named_attr!(
    #[doc="
        Recognize all operations expressions.
    "],
    pub operation<Span, Expression>,
    map_res!(
        logical,
        nary_expression_mapper
    )
);

#[inline]
fn nary_expression_mapper<'a>(nary_operation: NAryOperation<'a>) -> Result<Expression<'a>, ()> {
    Ok(Expression::NAryOperation(nary_operation))
}

macro_rules! left_to_right_binary_operation {
    (
        $parser_name:ident:
        $operand:ident with
        $operator_token:ident as $operator_representation:ident
    ) => (
        named!(
            $parser_name<Span, NAryOperation>,
            do_parse!(
                left_operand: $operand >>
                result: fold_many0!(
                    preceded!(
                        first!(tag!(tokens::$operator_token)),
                        first!($operand)
                    ),
                    left_operand,
                    |accumulator, right_operand| {
                        NAryOperation::Binary {
                            operator     : BinaryOperator::$operator_representation,
                            left_operand : Box::new(accumulator),
                            right_operand: Box::new(right_operand)
                        }
                    }
                ) >>
                (result)
            )
        );
    );

    (
        $parser_name:ident:
        $operand:ident with
        ($($operator_token:ident as $operator_representation:ident),*)
    ) => (
        named!(
            $parser_name<Span, NAryOperation>,
            do_parse!(
                left_operand: $operand >>
                result: fold_many0!(
                    do_parse!(
                        operator: first!(
                            alt_complete!(
                                $(
                                    tag!(tokens::$operator_token) => {
                                        |_| { BinaryOperator::$operator_representation }
                                    }
                                )|*
                            )
                        ) >>
                        right_operand: first!($operand) >>
                        (operator, right_operand)
                    ),
                    left_operand,
                    |accumulator, (operator, right_operand)| {
                        NAryOperation::Binary {
                            operator     : operator,
                            left_operand : Box::new(accumulator),
                            right_operand: Box::new(right_operand)
                        }
                    }
                ) >>
                (result)
            )
        );
    )
}

left_to_right_binary_operation!(
    logical:
    bitwise with
    (
        BOOLEAN_OR      as LogicalOr,
        BOOLEAN_AND     as LogicalAnd
    )
);

left_to_right_binary_operation!(
    bitwise:
    equality with
    (
        BITWISE_OR      as BitwiseOr,
        BITWISE_XOR     as BitwiseXor,
        BITWISE_AND     as BitwiseAnd
    )
);

left_to_right_binary_operation!(
    equality:
    relational with
    (
        EQUAL         as Equal,
        NOT_EQUAL     as NotEqual
    )
);

left_to_right_binary_operation!(
    relational:
    shift with
    (
        LESS_THAN_OR_EQUAL_TO    as LessThanOrEqualTo,
        GREATER_THAN_OR_EQUAL_TO as GreaterThanOrEqualTo,
        LESS_THAN                as LessThan,
        GREATER_THAN             as GreaterThan
    )
);

left_to_right_binary_operation!(
    shift:
    additive with
    (
        BITWISE_LEFT_SHIFT  as BitwiseShiftLeft,
        BITWISE_RIGHT_SHIFT as BitwiseShiftRight
    )
);

left_to_right_binary_operation!(
    additive:
    multiplicative with
    (
        ADD         as Addition,
        SUBSTRACT   as Subtraction
    )
);

left_to_right_binary_operation!(
    multiplicative:
    unary_operation with
    (
        MULTIPLY as Multiplication,
        DIVIDE   as Division,
        MODULO   as Modulo
    )
);

named!(
    unary_operation<Span, NAryOperation>,
    call!(leaf)
);

named!(
    leaf<Span, NAryOperation>,
    alt_complete!(
        map_res!(
            expression,
            |expression| -> Result<NAryOperation, ()> {
                Ok(NAryOperation::Nullary(Box::new(expression)))
            }
        )
        | preceded!(
            tag!(tokens::LEFT_PARENTHESIS),
            terminated!(
                first!(logical),
                first!(tag!(tokens::RIGHT_PARENTHESIS))
            )
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use ast::ast::{
        BinaryOperator,
        Expression,
        Literal,
        NAryOperation,
        Token,
        Variable
    };
    use span::Span;

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

    macro_rules! variable {
        ($name:expr) => (
            Expression::Variable(Variable($name))
        )
    }

    #[test]
    fn case_logical_or() {
        let input  = Span::new("1 || 2 || 3\n");
        let output = Ok((
            Span::new_at("\n", 11, 1, 12),
            Expression::NAryOperation(
                binary_operation!(
                    LogicalOr,
                    binary_operation!(
                        LogicalOr,
                        nullary_operation!(integer!(1, Span::new("1"))),
                        nullary_operation!(integer!(2, Span::new_at("2", 5, 1, 6)))
                    ),
                    nullary_operation!(integer!(3, Span::new_at("3", 10, 1, 11)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_logical_and() {
        let input  = Span::new("1 && 2 && 3\n");
        let output = Ok((
            Span::new_at("\n", 11, 1, 12),
            Expression::NAryOperation(
                binary_operation!(
                    LogicalAnd,
                    binary_operation!(
                        LogicalAnd,
                        nullary_operation!(integer!(1, Span::new("1"))),
                        nullary_operation!(integer!(2, Span::new_at("2", 5, 1, 6)))
                    ),
                    nullary_operation!(integer!(3, Span::new_at("3", 10, 1, 11)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_bitwise_or() {
        let input  = Span::new("1 | 2 | 3\n");
        let output = Ok((
            Span::new_at("\n", 9, 1, 10),
            Expression::NAryOperation(
                binary_operation!(
                    BitwiseOr,
                    binary_operation!(
                        BitwiseOr,
                        nullary_operation!(integer!(1, Span::new("1"))),
                        nullary_operation!(integer!(2, Span::new_at("2", 4, 1, 5)))
                    ),
                    nullary_operation!(integer!(3, Span::new_at("3", 8, 1, 9)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }


    #[test]
    fn case_bitwise_xor() {
        let input  = Span::new("1 ^ 2 ^ 3\n");
        let output = Ok((
            Span::new_at("\n", 9, 1, 10),
            Expression::NAryOperation(
                binary_operation!(
                    BitwiseXor,
                    binary_operation!(
                        BitwiseXor,
                        nullary_operation!(integer!(1, Span::new("1"))),
                        nullary_operation!(integer!(2, Span::new_at("2", 4, 1, 5)))
                    ),
                    nullary_operation!(integer!(3, Span::new_at("3", 8, 1, 9)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_bitwise_and() {
        let input  = Span::new("1 & 2 & 3\n");
        let output = Ok((
            Span::new_at("\n", 9, 1, 10),
            Expression::NAryOperation(
                binary_operation!(
                    BitwiseAnd,
                    binary_operation!(
                        BitwiseAnd,
                        nullary_operation!(integer!(1, Span::new("1"))),
                        nullary_operation!(integer!(2, Span::new_at("2", 4, 1, 5)))
                    ),
                    nullary_operation!(integer!(3, Span::new_at("3", 8, 1, 9)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_equality_equal() {
        let input  = Span::new("1 == 2 == 3\n");
        let output = Ok((
            Span::new_at("\n", 11, 1, 12),
            Expression::NAryOperation(
                binary_operation!(
                    Equal,
                    binary_operation!(
                        Equal,
                        nullary_operation!(integer!(1, Span::new("1"))),
                        nullary_operation!(integer!(2, Span::new_at("2", 5, 1, 6)))
                    ),
                    nullary_operation!(integer!(3, Span::new_at("3", 10, 1, 11)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_equality_not_equal() {
        let input  = Span::new("1 != 2 != 3\n");
        let output = Ok((
            Span::new_at("\n", 11, 1, 12),
            Expression::NAryOperation(
                binary_operation!(
                    NotEqual,
                    binary_operation!(
                        NotEqual,
                        nullary_operation!(integer!(1, Span::new("1"))),
                        nullary_operation!(integer!(2, Span::new_at("2", 5, 1, 6)))
                    ),
                    nullary_operation!(integer!(3, Span::new_at("3", 10, 1, 11)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_relational_less_than_or_equal_to() {
        let input  = Span::new("1 <= 2 <= 3\n");
        let output = Ok((
            Span::new_at("\n", 11, 1, 12),
            Expression::NAryOperation(
                binary_operation!(
                    LessThanOrEqualTo,
                    binary_operation!(
                        LessThanOrEqualTo,
                        nullary_operation!(integer!(1, Span::new("1"))),
                        nullary_operation!(integer!(2, Span::new_at("2", 5, 1, 6)))
                    ),
                    nullary_operation!(integer!(3, Span::new_at("3", 10, 1, 11)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_relational_greater_than_or_equal_to() {
        let input  = Span::new("1 >= 2 >= 3\n");
        let output = Ok((
            Span::new_at("\n", 11, 1, 12),
            Expression::NAryOperation(
                binary_operation!(
                    GreaterThanOrEqualTo,
                    binary_operation!(
                        GreaterThanOrEqualTo,
                        nullary_operation!(integer!(1, Span::new("1"))),
                        nullary_operation!(integer!(2, Span::new_at("2", 5, 1, 6)))
                    ),
                    nullary_operation!(integer!(3, Span::new_at("3", 10, 1, 11)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_relational_less_than() {
        let input  = Span::new("1 < 2 < 3\n");
        let output = Ok((
            Span::new_at("\n", 9, 1, 10),
            Expression::NAryOperation(
                binary_operation!(
                    LessThan,
                    binary_operation!(
                        LessThan,
                        nullary_operation!(integer!(1, Span::new("1"))),
                        nullary_operation!(integer!(2, Span::new_at("2", 4, 1, 5)))
                    ),
                    nullary_operation!(integer!(3, Span::new_at("3", 8, 1, 9)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_relational_greater_than() {
        let input  = Span::new("1 > 2 > 3\n");
        let output = Ok((
            Span::new_at("\n", 9, 1, 10),
            Expression::NAryOperation(
                binary_operation!(
                    GreaterThan,
                    binary_operation!(
                        GreaterThan,
                        nullary_operation!(integer!(1, Span::new("1"))),
                        nullary_operation!(integer!(2, Span::new_at("2", 4, 1, 5)))
                    ),
                    nullary_operation!(integer!(3, Span::new_at("3", 8, 1, 9)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_shift_left() {
        let input  = Span::new("1 << 2 << 3\n");
        let output = Ok((
            Span::new_at("\n", 11, 1, 12),
            Expression::NAryOperation(
                binary_operation!(
                    BitwiseShiftLeft,
                    binary_operation!(
                        BitwiseShiftLeft,
                        nullary_operation!(integer!(1, Span::new("1"))),
                        nullary_operation!(integer!(2, Span::new_at("2", 5, 1, 6)))
                    ),
                    nullary_operation!(integer!(3, Span::new_at("3", 10, 1, 11)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_shift_right() {
        let input  = Span::new("1 >> 2 >> 3\n");
        let output = Ok((
            Span::new_at("\n", 11, 1, 12),
            Expression::NAryOperation(
                binary_operation!(
                    BitwiseShiftRight,
                    binary_operation!(
                        BitwiseShiftRight,
                        nullary_operation!(integer!(1, Span::new("1"))),
                        nullary_operation!(integer!(2, Span::new_at("2", 5, 1, 6)))
                    ),
                    nullary_operation!(integer!(3, Span::new_at("3", 10, 1, 11)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_additive_add() {
        let input  = Span::new("1 + 2 + 3\n");
        let output = Ok((
            Span::new_at("\n", 9, 1, 10),
            Expression::NAryOperation(
                binary_operation!(
                    Addition,
                    binary_operation!(
                        Addition,
                        nullary_operation!(integer!(1, Span::new("1"))),
                        nullary_operation!(integer!(2, Span::new_at("2", 4, 1, 5)))
                    ),
                    nullary_operation!(integer!(3, Span::new_at("3", 8, 1, 9)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_additive_substract() {
        let input  = Span::new("1 - 2 - 3\n");
        let output = Ok((
            Span::new_at("\n", 9, 1, 10),
            Expression::NAryOperation(
                binary_operation!(
                    Subtraction,
                    binary_operation!(
                        Subtraction,
                        nullary_operation!(integer!(1, Span::new("1"))),
                        nullary_operation!(integer!(2, Span::new_at("2", 4, 1, 5)))
                    ),
                    nullary_operation!(integer!(3, Span::new_at("3", 8, 1, 9)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_multiplicative_multiply() {
        let input  = Span::new("1 * 2 * 3\n");
        let output = Ok((
            Span::new_at("\n", 9, 1, 10),
            Expression::NAryOperation(
                binary_operation!(
                    Multiplication,
                    binary_operation!(
                        Multiplication,
                        nullary_operation!(integer!(1, Span::new("1"))),
                        nullary_operation!(integer!(2, Span::new_at("2", 4, 1, 5)))
                    ),
                    nullary_operation!(integer!(3, Span::new_at("3", 8, 1, 9)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_multiplicative_division() {
        let input  = Span::new("1 / 2 / 3\n");
        let output = Ok((
            Span::new_at("\n", 9, 1, 10),
            Expression::NAryOperation(
                binary_operation!(
                    Division,
                    binary_operation!(
                        Division,
                        nullary_operation!(integer!(1, Span::new("1"))),
                        nullary_operation!(integer!(2, Span::new_at("2", 4, 1, 5)))
                    ),
                    nullary_operation!(integer!(3, Span::new_at("3", 8, 1, 9)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_multiplicative_modulo() {
        let input  = Span::new("1 % 2 % 3\n");
        let output = Ok((
            Span::new_at("\n", 9, 1, 10),
            Expression::NAryOperation(
                binary_operation!(
                    Modulo,
                    binary_operation!(
                        Modulo,
                        nullary_operation!(integer!(1, Span::new("1"))),
                        nullary_operation!(integer!(2, Span::new_at("2", 4, 1, 5)))
                    ),
                    nullary_operation!(integer!(3, Span::new_at("3", 8, 1, 9)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_precedence_additive_multiplicative() {
        let input  = Span::new("1 + 2 * 3\n");
        let output = Ok((
            Span::new_at("\n", 9, 1, 10),
            Expression::NAryOperation(
                binary_operation!(
                    Addition,
                    nullary_operation!(integer!(1, Span::new_at("1", 0, 1, 1))),
                    binary_operation!(
                        Multiplication,
                        nullary_operation!(integer!(2, Span::new_at("2", 4, 1, 5))),
                        nullary_operation!(integer!(3, Span::new_at("3", 8, 1, 9)))
                    )
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_precedence_logical_equality() {
        let input  = Span::new("1 == 2 || 3 != 4\n");
        let output = Ok((
            Span::new_at("\n", 16, 1, 17),
            Expression::NAryOperation(
                binary_operation!(
                    LogicalOr,
                    binary_operation!(
                        Equal,
                        nullary_operation!(integer!(1, Span::new_at("1", 0, 1, 1))),
                        nullary_operation!(integer!(2, Span::new_at("2", 5, 1, 6)))
                    ),
                    binary_operation!(
                        NotEqual,
                        nullary_operation!(integer!(3, Span::new_at("3", 10, 1, 11))),
                        nullary_operation!(integer!(4, Span::new_at("4", 15, 1, 16)))
                    )
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_precedence_equality_additive() {
        let input  = Span::new("1 + 2 == 3 - 4\n");
        let output = Ok((
            Span::new_at("\n", 14, 1, 15),
            Expression::NAryOperation(
                binary_operation!(
                    Equal,
                    binary_operation!(
                        Addition,
                        nullary_operation!(integer!(1, Span::new_at("1", 0, 1, 1))),
                        nullary_operation!(integer!(2, Span::new_at("2", 4, 1, 5)))
                    ),
                    binary_operation!(
                        Subtraction,
                        nullary_operation!(integer!(3, Span::new_at("3", 9, 1, 10))),
                        nullary_operation!(integer!(4, Span::new_at("4", 13, 1, 14)))
                    )
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_whitespace() {
        let input  = Span::new("1   + 2 *   3\n");
        let output = Ok((
            Span::new_at("\n", 13, 1, 14),
            Expression::NAryOperation(
                binary_operation!(
                    Addition,
                    nullary_operation!(integer!(1, Span::new_at("1", 0, 1, 1))),
                    binary_operation!(
                        Multiplication,
                        nullary_operation!(integer!(2, Span::new_at("2", 6, 1, 7))),
                        nullary_operation!(integer!(3, Span::new_at("3", 12, 1, 13)))
                    )
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_variables() {
        let input  = Span::new("1 + x * y\n");
        let output = Ok((
            Span::new_at("\n", 9, 1, 10),
            Expression::NAryOperation(
                binary_operation!(
                    Addition,
                    nullary_operation!(integer!(1, Span::new_at("1", 0, 1, 1))),
                    binary_operation!(
                        Multiplication,
                        nullary_operation!(variable!(Span::new_at("x", 4, 1, 5))),
                        nullary_operation!(variable!(Span::new_at("y",  8, 1, 9)))
                    )
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_parenthesis() {
        let input  = Span::new("(((1 + 2) * ((3))))\n");
        let output = Ok((
            Span::new_at("\n", 19, 1, 20),
            Expression::NAryOperation(
                binary_operation!(
                    Multiplication,
                    binary_operation!(
                        Addition,
                        nullary_operation!(integer!(1, Span::new_at("1", 3, 1, 4))),
                        nullary_operation!(integer!(2, Span::new_at("2",  7, 1, 8)))
                    ),
                    nullary_operation!(integer!(3, Span::new_at("3", 14, 1, 15)))

                )
            )
        ));

        assert_eq!(operation(input), output);
    }
}