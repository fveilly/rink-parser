use span::Span;
use tokens;
use ast::ast::{
    NAryOperation,
    BinaryOperator,
    UnaryOperator,
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

macro_rules! left_to_right_unary_operation {
    (
        $parser_name:ident:
        $operand:ident with
        $operator_token:ident as $operator_representation:ident
    ) => (
        named!(
            $parser_name<Span, NAryOperation>,
            do_parse!(
                left_operand: $operand >>
                unary_operator: opt!(tag!(tokens::$operator_token)) >>
                (
                    match (unary_operator) {
                        Some(operator) => {
                            NAryOperation::Unary {
                                operator : operator,
                                operand  : Box::new(left_operand)
                            }
                        },
                        _ => left_operand
                    }
                )
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
                unary_operator: opt!(alt_complete!(
                        $(
                            tag!(tokens::$operator_token) => {
                                |_| { UnaryOperator::$operator_representation }
                            }
                        )|*
                    )
                ) >>
                (
                    match (unary_operator) {
                        Some(operator) => {
                            NAryOperation::Unary {
                                operator : operator,
                                operand  : Box::new(left_operand)
                            }
                        },
                        _ => left_operand
                    }
                )
            )
        );
    )
}

macro_rules! right_to_left_unary_operation {
    (
        $parser_name:ident:
        $operand:ident with
        $operator_token:ident as $operator_representation:ident
    ) => (
        named!(
            $parser_name<Span, NAryOperation>,
            alt_complete!(
                do_parse!(
                    tag!(tokens::$operator_token) >>
                    operand: $parser_name >>
                    (
                        NAryOperation::Unary {
                            operator : BinaryOperator::$operator_representation,
                            operand  : operand
                        }
                    )
                )
                | $operand
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
            alt_complete!(
                do_parse!(
                    operator: alt_complete!(
                        $(
                            tag!(tokens::$operator_token) => {
                                |_| { UnaryOperator::$operator_representation }
                            }
                        )|*
                    ) >>
                    operand: $parser_name >>
                    (
                        NAryOperation::Unary {
                            operator : operator,
                            operand  : Box::new(operand)
                        }
                    )
                )
                | $operand
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
    unary_not with
    (
        MULTIPLY as Multiplication,
        DIVIDE   as Division,
        MODULO   as Modulo
    )
);

right_to_left_unary_operation!(
    unary_not:
    increment_and_decrement with
    (
        BITWISE_NOT as BitwiseComplement,
        BOOLEAN_NOT as Negate
    )
);

left_to_right_unary_operation!(
    increment_and_decrement:
    nullary_operation with
    (
        INCREMENT as Increment,
        DECREMENT as Decrement
    )
);

named!(
    nullary_operation<Span, NAryOperation>,
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

    macro_rules! nullary {
        ($expression:expr) => (
            NAryOperation::Nullary(Box::new($expression))
        )
    }

    macro_rules! unary {
        ($operator:ident, $operand:expr) => (
            NAryOperation::Unary {
                operator : UnaryOperator::$operator,
                operand  : Box::new($operand)
            }
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

    macro_rules! variable {
        ($name:expr) => (
            Expression::Variable(Variable($name))
        )
    }

    #[test]
    fn case_binary_logical_or() {
        let input  = Span::new("1 || 2 || 3\n");
        let output = Ok((
            Span::new_at("\n", 11, 1, 12),
            Expression::NAryOperation(
                binary!(
                    LogicalOr,
                    binary!(
                        LogicalOr,
                        nullary!(integer!(1, Span::new("1"))),
                        nullary!(integer!(2, Span::new_at("2", 5, 1, 6)))
                    ),
                    nullary!(integer!(3, Span::new_at("3", 10, 1, 11)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_binary_logical_and() {
        let input  = Span::new("1 && 2 && 3\n");
        let output = Ok((
            Span::new_at("\n", 11, 1, 12),
            Expression::NAryOperation(
                binary!(
                    LogicalAnd,
                    binary!(
                        LogicalAnd,
                        nullary!(integer!(1, Span::new("1"))),
                        nullary!(integer!(2, Span::new_at("2", 5, 1, 6)))
                    ),
                    nullary!(integer!(3, Span::new_at("3", 10, 1, 11)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_binary_bitwise_or() {
        let input  = Span::new("1 | 2 | 3\n");
        let output = Ok((
            Span::new_at("\n", 9, 1, 10),
            Expression::NAryOperation(
                binary!(
                    BitwiseOr,
                    binary!(
                        BitwiseOr,
                        nullary!(integer!(1, Span::new("1"))),
                        nullary!(integer!(2, Span::new_at("2", 4, 1, 5)))
                    ),
                    nullary!(integer!(3, Span::new_at("3", 8, 1, 9)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }


    #[test]
    fn case_binary_bitwise_xor() {
        let input  = Span::new("1 ^ 2 ^ 3\n");
        let output = Ok((
            Span::new_at("\n", 9, 1, 10),
            Expression::NAryOperation(
                binary!(
                    BitwiseXor,
                    binary!(
                        BitwiseXor,
                        nullary!(integer!(1, Span::new("1"))),
                        nullary!(integer!(2, Span::new_at("2", 4, 1, 5)))
                    ),
                    nullary!(integer!(3, Span::new_at("3", 8, 1, 9)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_binary_bitwise_and() {
        let input  = Span::new("1 & 2 & 3\n");
        let output = Ok((
            Span::new_at("\n", 9, 1, 10),
            Expression::NAryOperation(
                binary!(
                    BitwiseAnd,
                    binary!(
                        BitwiseAnd,
                        nullary!(integer!(1, Span::new("1"))),
                        nullary!(integer!(2, Span::new_at("2", 4, 1, 5)))
                    ),
                    nullary!(integer!(3, Span::new_at("3", 8, 1, 9)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_binary_equality_equal() {
        let input  = Span::new("1 == 2 == 3\n");
        let output = Ok((
            Span::new_at("\n", 11, 1, 12),
            Expression::NAryOperation(
                binary!(
                    Equal,
                    binary!(
                        Equal,
                        nullary!(integer!(1, Span::new("1"))),
                        nullary!(integer!(2, Span::new_at("2", 5, 1, 6)))
                    ),
                    nullary!(integer!(3, Span::new_at("3", 10, 1, 11)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_binary_equality_not_equal() {
        let input  = Span::new("1 != 2 != 3\n");
        let output = Ok((
            Span::new_at("\n", 11, 1, 12),
            Expression::NAryOperation(
                binary!(
                    NotEqual,
                    binary!(
                        NotEqual,
                        nullary!(integer!(1, Span::new("1"))),
                        nullary!(integer!(2, Span::new_at("2", 5, 1, 6)))
                    ),
                    nullary!(integer!(3, Span::new_at("3", 10, 1, 11)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_binary_relational_less_than_or_equal_to() {
        let input  = Span::new("1 <= 2 <= 3\n");
        let output = Ok((
            Span::new_at("\n", 11, 1, 12),
            Expression::NAryOperation(
                binary!(
                    LessThanOrEqualTo,
                    binary!(
                        LessThanOrEqualTo,
                        nullary!(integer!(1, Span::new("1"))),
                        nullary!(integer!(2, Span::new_at("2", 5, 1, 6)))
                    ),
                    nullary!(integer!(3, Span::new_at("3", 10, 1, 11)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_binary_relational_greater_than_or_equal_to() {
        let input  = Span::new("1 >= 2 >= 3\n");
        let output = Ok((
            Span::new_at("\n", 11, 1, 12),
            Expression::NAryOperation(
                binary!(
                    GreaterThanOrEqualTo,
                    binary!(
                        GreaterThanOrEqualTo,
                        nullary!(integer!(1, Span::new("1"))),
                        nullary!(integer!(2, Span::new_at("2", 5, 1, 6)))
                    ),
                    nullary!(integer!(3, Span::new_at("3", 10, 1, 11)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_binary_relational_less_than() {
        let input  = Span::new("1 < 2 < 3\n");
        let output = Ok((
            Span::new_at("\n", 9, 1, 10),
            Expression::NAryOperation(
                binary!(
                    LessThan,
                    binary!(
                        LessThan,
                        nullary!(integer!(1, Span::new("1"))),
                        nullary!(integer!(2, Span::new_at("2", 4, 1, 5)))
                    ),
                    nullary!(integer!(3, Span::new_at("3", 8, 1, 9)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_binary_relational_greater_than() {
        let input  = Span::new("1 > 2 > 3\n");
        let output = Ok((
            Span::new_at("\n", 9, 1, 10),
            Expression::NAryOperation(
                binary!(
                    GreaterThan,
                    binary!(
                        GreaterThan,
                        nullary!(integer!(1, Span::new("1"))),
                        nullary!(integer!(2, Span::new_at("2", 4, 1, 5)))
                    ),
                    nullary!(integer!(3, Span::new_at("3", 8, 1, 9)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_binary_shift_left() {
        let input  = Span::new("1 << 2 << 3\n");
        let output = Ok((
            Span::new_at("\n", 11, 1, 12),
            Expression::NAryOperation(
                binary!(
                    BitwiseShiftLeft,
                    binary!(
                        BitwiseShiftLeft,
                        nullary!(integer!(1, Span::new("1"))),
                        nullary!(integer!(2, Span::new_at("2", 5, 1, 6)))
                    ),
                    nullary!(integer!(3, Span::new_at("3", 10, 1, 11)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_binary_shift_right() {
        let input  = Span::new("1 >> 2 >> 3\n");
        let output = Ok((
            Span::new_at("\n", 11, 1, 12),
            Expression::NAryOperation(
                binary!(
                    BitwiseShiftRight,
                    binary!(
                        BitwiseShiftRight,
                        nullary!(integer!(1, Span::new("1"))),
                        nullary!(integer!(2, Span::new_at("2", 5, 1, 6)))
                    ),
                    nullary!(integer!(3, Span::new_at("3", 10, 1, 11)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_binary_additive_add() {
        let input  = Span::new("1 + 2 + 3\n");
        let output = Ok((
            Span::new_at("\n", 9, 1, 10),
            Expression::NAryOperation(
                binary!(
                    Addition,
                    binary!(
                        Addition,
                        nullary!(integer!(1, Span::new("1"))),
                        nullary!(integer!(2, Span::new_at("2", 4, 1, 5)))
                    ),
                    nullary!(integer!(3, Span::new_at("3", 8, 1, 9)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_binary_additive_substract() {
        let input  = Span::new("1 - 2 - 3\n");
        let output = Ok((
            Span::new_at("\n", 9, 1, 10),
            Expression::NAryOperation(
                binary!(
                    Subtraction,
                    binary!(
                        Subtraction,
                        nullary!(integer!(1, Span::new("1"))),
                        nullary!(integer!(2, Span::new_at("2", 4, 1, 5)))
                    ),
                    nullary!(integer!(3, Span::new_at("3", 8, 1, 9)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_binary_multiplicative_multiply() {
        let input  = Span::new("1 * 2 * 3\n");
        let output = Ok((
            Span::new_at("\n", 9, 1, 10),
            Expression::NAryOperation(
                binary!(
                    Multiplication,
                    binary!(
                        Multiplication,
                        nullary!(integer!(1, Span::new("1"))),
                        nullary!(integer!(2, Span::new_at("2", 4, 1, 5)))
                    ),
                    nullary!(integer!(3, Span::new_at("3", 8, 1, 9)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_binary_multiplicative_division() {
        let input  = Span::new("1 / 2 / 3\n");
        let output = Ok((
            Span::new_at("\n", 9, 1, 10),
            Expression::NAryOperation(
                binary!(
                    Division,
                    binary!(
                        Division,
                        nullary!(integer!(1, Span::new("1"))),
                        nullary!(integer!(2, Span::new_at("2", 4, 1, 5)))
                    ),
                    nullary!(integer!(3, Span::new_at("3", 8, 1, 9)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_binary_multiplicative_modulo() {
        let input  = Span::new("1 % 2 % 3\n");
        let output = Ok((
            Span::new_at("\n", 9, 1, 10),
            Expression::NAryOperation(
                binary!(
                    Modulo,
                    binary!(
                        Modulo,
                        nullary!(integer!(1, Span::new("1"))),
                        nullary!(integer!(2, Span::new_at("2", 4, 1, 5)))
                    ),
                    nullary!(integer!(3, Span::new_at("3", 8, 1, 9)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_binary_precedence_additive_multiplicative() {
        let input  = Span::new("1 + 2 * 3\n");
        let output = Ok((
            Span::new_at("\n", 9, 1, 10),
            Expression::NAryOperation(
                binary!(
                    Addition,
                    nullary!(integer!(1, Span::new_at("1", 0, 1, 1))),
                    binary!(
                        Multiplication,
                        nullary!(integer!(2, Span::new_at("2", 4, 1, 5))),
                        nullary!(integer!(3, Span::new_at("3", 8, 1, 9)))
                    )
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_binary_precedence_logical_equality() {
        let input  = Span::new("1 == 2 || 3 != 4\n");
        let output = Ok((
            Span::new_at("\n", 16, 1, 17),
            Expression::NAryOperation(
                binary!(
                    LogicalOr,
                    binary!(
                        Equal,
                        nullary!(integer!(1, Span::new_at("1", 0, 1, 1))),
                        nullary!(integer!(2, Span::new_at("2", 5, 1, 6)))
                    ),
                    binary!(
                        NotEqual,
                        nullary!(integer!(3, Span::new_at("3", 10, 1, 11))),
                        nullary!(integer!(4, Span::new_at("4", 15, 1, 16)))
                    )
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_binary_precedence_equality_additive() {
        let input  = Span::new("1 + 2 == 3 - 4\n");
        let output = Ok((
            Span::new_at("\n", 14, 1, 15),
            Expression::NAryOperation(
                binary!(
                    Equal,
                    binary!(
                        Addition,
                        nullary!(integer!(1, Span::new_at("1", 0, 1, 1))),
                        nullary!(integer!(2, Span::new_at("2", 4, 1, 5)))
                    ),
                    binary!(
                        Subtraction,
                        nullary!(integer!(3, Span::new_at("3", 9, 1, 10))),
                        nullary!(integer!(4, Span::new_at("4", 13, 1, 14)))
                    )
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_binary_whitespace() {
        let input  = Span::new("1   + 2 *   3\n");
        let output = Ok((
            Span::new_at("\n", 13, 1, 14),
            Expression::NAryOperation(
                binary!(
                    Addition,
                    nullary!(integer!(1, Span::new_at("1", 0, 1, 1))),
                    binary!(
                        Multiplication,
                        nullary!(integer!(2, Span::new_at("2", 6, 1, 7))),
                        nullary!(integer!(3, Span::new_at("3", 12, 1, 13)))
                    )
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_binary_variables() {
        let input  = Span::new("1 + x * y\n");
        let output = Ok((
            Span::new_at("\n", 9, 1, 10),
            Expression::NAryOperation(
                binary!(
                    Addition,
                    nullary!(integer!(1, Span::new_at("1", 0, 1, 1))),
                    binary!(
                        Multiplication,
                        nullary!(variable!(Span::new_at("x", 4, 1, 5))),
                        nullary!(variable!(Span::new_at("y",  8, 1, 9)))
                    )
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_binary_parenthesis() {
        let input  = Span::new("(((1 + 2) * ((3))))\n");
        let output = Ok((
            Span::new_at("\n", 19, 1, 20),
            Expression::NAryOperation(
                binary!(
                    Multiplication,
                    binary!(
                        Addition,
                        nullary!(integer!(1, Span::new_at("1", 3, 1, 4))),
                        nullary!(integer!(2, Span::new_at("2",  7, 1, 8)))
                    ),
                    nullary!(integer!(3, Span::new_at("3", 14, 1, 15)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_unary_increment() {
        let input  = Span::new("1++\n");
        let output = Ok((
            Span::new_at("\n", 3, 1, 4),
            Expression::NAryOperation(
                unary!(
                    Increment,
                    nullary!(integer!(1, Span::new_at("1", 0, 1, 1)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_unary_multiple_increment() {
        let input  = Span::new("(1++)++\n");
        let output = Ok((
            Span::new_at("\n", 7, 1, 8),
            Expression::NAryOperation(
                unary!(
                    Increment,
                    unary!(
                        Increment,
                        nullary!(integer!(1, Span::new_at("1", 1, 1, 2)))
                    )
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_unary_decrement() {
        let input  = Span::new("1--\n");
        let output = Ok((
            Span::new_at("\n", 3, 1, 4),
            Expression::NAryOperation(
                unary!(
                    Decrement,
                    nullary!(integer!(1, Span::new_at("1", 0, 1, 1)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_unary_bitwise_complement() {
        let input  = Span::new("~x\n");
        let output = Ok((
            Span::new_at("\n", 2, 1, 3),
            Expression::NAryOperation(
                unary!(
                    BitwiseComplement,
                    nullary!(variable!(Span::new_at("x", 1, 1, 2)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_unary_negate() {
        let input  = Span::new("!x\n");
        let output = Ok((
            Span::new_at("\n", 2, 1, 3),
            Expression::NAryOperation(
                unary!(
                    Negate,
                    nullary!(variable!(Span::new_at("x", 1, 1, 2)))
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_unary_multiple_negate() {
        let input  = Span::new("!!!x\n");
        let output = Ok((
            Span::new_at("\n", 4, 1, 5),
            Expression::NAryOperation(
                unary!(
                    Negate,
                    unary!(
                        Negate,
                        unary!(
                            Negate,
                            nullary!(variable!(Span::new_at("x", 3, 1, 4)))
                        )
                    )
                )
            )
        ));

        assert_eq!(operation(input), output);
    }

    #[test]
    fn case_unary_precedence() {
        let input  = Span::new("~!x++\n");
        let output = Ok((
            Span::new_at("\n", 5, 1, 6),
            Expression::NAryOperation(
                unary!(
                    BitwiseComplement,
                    unary!(
                        Negate,
                        unary!(
                            Increment,
                            nullary!(variable!(Span::new_at("x", 2, 1, 3)))
                        )
                    )
                )
            )
        ));

        assert_eq!(operation(input), output);
    }
}