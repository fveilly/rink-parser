use ast::ast::{
    Literal,
    Token
};

use span::Span;
use internal::ErrorKindExtension;
use tokens;

use std::result::Result as StdResult;

use std::num::{
    ParseFloatError,
    ParseIntError
};

use std::str::{
    FromStr,
    ParseBoolError
};

use nom::{
    InputIter,
    InputLength,
    Slice,
    IResult,
    AsChar,
    AtEof,
    Compare,
    CompareResult,
    hex_digit,
    oct_digit,
    digit
};

named_attr!(
    #[doc="
        Recognize a number.
        A number is either a boolean, an integer or a real.
    "],
    pub number<Span, Literal>,
    alt!(
        boolean
        | real
        | integer
    )
);

named_attr!(
    #[doc="
        Recognize an integer.
        An integer is either a binary, a decimal, an hexadecimal or an octal representation.
    "],
    pub integer<Span, Literal>,
    alt_complete!(
        binary
        | hexadecimal
        | decimal
        | octal
    )
);

#[inline]
pub fn is_binary_digit(chr: char) -> bool {
    chr == '0' || chr == '1'
}

/// Recognizes one or more binary numerical characters
pub fn binary_digit(input: Span) -> IResult<Span, Span> {
    use nom::{
        Err,
        ErrorKind,
        Needed
    };

    match input.position(|item| !is_binary_digit(item)) {
        Some(0) => Err(Err::Error(error_position!(input, ErrorKind::Custom(ErrorKindExtension::BinaryDigit as u32)))),
        Some(n) => Ok((input.slice(n..), input.slice(..n))),
        None => {
            if input.at_eof() {
                if input.input_len() > 0 {
                    Ok((input.slice(input.input_len()..), input))
                } else {
                    Err(Err::Error(error_position!(
                        input,
                        ErrorKind::Custom(ErrorKindExtension::BinaryDigit as u32)
                      )))
                }
            } else {
                Err(Err::Incomplete(Needed::Size(1)))
            }
        }
    }
}

/// Recognizes one or more decimal characters. A decimal integer literal (base ten) begins with a digit
/// other than 0 and consists of a sequence of decimal digits. 0 is considered as a decimal.
pub fn decimal_digit(input: Span) -> IResult<Span, Span> {
    use nom::{
        Err,
        ErrorKind,
        Needed
    };

    match input.position(|item| !item.is_dec_digit()) {
        Some(0) => Err(Err::Error(error_position!(input, ErrorKind::Digit))),
        Some(1) => Ok((input.slice(1..), input.slice(..1))),
        Some(n) => {
            match input.compare('0') {
                CompareResult::Error => { Ok((input.slice(n..), input.slice(..n))) },
                _ => { Err(Err::Error(error_position!(input, ErrorKind::Digit))) }
            }
        },
        None => {
            if input.at_eof() {
                if input.input_len() > 0 {
                    Ok((input.slice(input.input_len()..), input))
                } else {
                    Err(Err::Error(error_position!(input, ErrorKind::Digit)))
                }
            } else {
                Err(Err::Incomplete(Needed::Size(1)))
            }
        }
    }
}

named_attr!(
    #[doc="
        Recognize an exponential notation.
    "],
    pub exponential<Span, Span>,
    recognize!(
        tuple!(
            alt!(char!('e') | char!('E')),
            opt!(alt!(char!('+') | char!('-'))),
            digit
        )
    )
);

/// Recognizes a real digit. Can be one of these forms.
/// [0-9]*\.[0-9]+([eE][+-]?[0-9]+)?
/// [0-9]+\.([eE][+-]?[0-9]+)?
/// [0-9]+[eE][+-]?[0-9]+)
pub fn real_digit(input: Span) -> IResult<Span, Span> {
    use nom::digit;

    recognize!(input,
        alt!(
            value!((), tuple!(opt!(digit), char!('.'), digit, opt!(exponential)))
            | value!((), tuple!(digit, char!('.'), opt!(exponential)))
            | value!((), tuple!(digit, exponential))
        )
    )
}

named_attr!(
    #[doc="
        Recognize an integer with the binary notation.
    "],
    pub binary<Span, Literal>,
    map_res!(
        preceded!(
            tag!("0b"),
            call!(binary_digit)
        ),
        binary_mapper
    )
);

#[inline]
fn binary_mapper(span: Span) -> StdResult<Literal, ParseIntError> {
    i64::from_str_radix(span.as_slice(), 2)
        .and_then(
            | binary| {
                Ok(Literal::Integer(Token::new(binary, span)))
            }
        )
}

named_attr!(
    #[doc="
        Recognize an integer with the hexadecimal notation.
    "],
    pub hexadecimal<Span, Literal>,
    map_res!(
        preceded!(
            tag!("0x"),
            call!(hex_digit)
        ),
        hexadecimal_mapper
    )
);

#[inline]
fn hexadecimal_mapper(span: Span) -> StdResult<Literal, ParseIntError> {
    i64::from_str_radix(span.as_slice(), 16)
        .and_then(
            | hex| {
                Ok(Literal::Integer(Token::new(hex, span)))
            }
        )
}

named_attr!(
    #[doc="
        Recognize an integer with the octal notation.
    "],
    pub octal<Span, Literal>,
    map_res!(
        preceded!(
            tag!("0"),
            call!(oct_digit)
        ),
        octal_mapper
    )
);

#[inline]
fn octal_mapper(span: Span) -> StdResult<Literal, ParseIntError> {
    i64::from_str_radix(span.as_slice(), 8)
        .and_then(
            | octal| {
                Ok(Literal::Integer(Token::new(octal, span)))
            }
        )
}

named_attr!(
    #[doc="
        Recognize an integer with the decimal notation.
    "],
    pub decimal<Span, Literal>,
    map_res!(
        call!(decimal_digit),
        decimal_mapper
    )
);

#[inline]
fn decimal_mapper(span: Span) -> StdResult<Literal, ParseFloatError> {
    i64::from_str(span.as_slice())
        .and_then(
            | decimal| {
                Ok(Literal::Integer(Token::new(decimal, span)))
            }
        ).or_else(
            |_: ParseIntError| {
                f64::from_str(span.as_slice())
                    .and_then(
                        | decimal | {
                            Ok(Literal::Real(Token::new(decimal, span)))
                        }
                    )
            }
    )
}

named_attr!(
    #[doc="
        Recognize a real number.
    "],
    pub real<Span, Literal>,
    map_res!(
        call!(real_digit),
        real_mapper
    )
);

#[inline]
fn real_mapper(span: Span) -> StdResult<Literal, ParseFloatError> {
    f64::from_str(span.as_slice())
        .and_then(
            | decimal | {
                Ok(Literal::Real(Token::new(decimal, span)))
            }
        )
}

named_attr!(
    #[doc="
        Recognize a boolean.
    "],
    pub boolean<Span, Literal>,
    map_res!(
        recognize!(
            alt!(tag!(tokens::TRUE) | tag!(tokens::FALSE))
        ),
        boolean_mapper
    )
);

#[inline]
fn boolean_mapper(span: Span) -> StdResult<Literal, ParseBoolError> {
    bool::from_str(span.as_slice())
        .and_then(
            | boolean | {
                Ok(Literal::Boolean(Token::new(boolean, span)))
            }
        )
}

#[cfg(test)]
mod tests {
    use super::{
        integer,
        binary,
        hexadecimal,
        octal,
        decimal,
        real,
        boolean
    };

    use ast::ast::{
        Literal,
        Token
    };

    use internal::{
        Context,
        Error,
        ErrorKind
    };

    use span::Span;
    use internal::ErrorKindExtension;

    #[test]
    fn case_binary() {
        let input  = Span::new("0b101010\n");
        let output = Ok((
            Span::new_at("\n", 8, 1, 9),
            Literal::Integer(Token::new(42i64, Span::new_at("101010", 2, 1, 3)))
        ));

        assert_eq!(binary(input),  output);
        assert_eq!(integer(input), output);
    }

    #[test]
    fn case_binary_maximum_integer_value() {
        let input  = Span::new("0b111111111111111111111111111111111111111111111111111111111111111\n");
        let output = Ok((
            Span::new_at("\n", 65, 1, 66),
            Literal::Integer(Token::new(::std::i64::MAX,
                                        Span::new_at("111111111111111111111111111111111111111111111111111111111111111",
                                                     2, 1, 3)))
        ));

        assert_eq!(binary(input), output);
        assert_eq!(integer(input), output);
    }

    #[test]
    fn case_invalid_binary_overflow() {
        let input  = Span::new("0b1000000000000000000000000000000000000000000000000000000000000000\n");
        let output = Ok((
            Span::new_at("b1000000000000000000000000000000000000000000000000000000000000000\n", 1, 1, 2),
            Literal::Integer(Token::new(0i64, Span::new("0")))
        ));

        assert_eq!(binary(input), Err(Error::Error(Context::Code(input, ErrorKind::MapRes))));
        assert_eq!(integer(input),  output);
    }

    #[test]
    fn case_invalid_binary_no_number() {
        let input  = Span::new("0b\n");
        let output =  Ok((
            Span::new_at("b\n", 1, 1, 2),
            Literal::Integer(Token::new(0, Span::new_at("0", 0, 1, 1)))
        ));

        assert_eq!(binary(input), Err(Error::Error(Context::Code( Span::new_at("\n", 2, 1, 3),
                                                                  ErrorKind::Custom(ErrorKindExtension::BinaryDigit as u32)))));
        assert_eq!(integer(input), output);
    }

    #[test]
    fn case_invalid_binary_not_starting_by_zero_b() {
        let input  = Span::new("1\n");
        let output =  Ok((
            Span::new_at("\n", 1, 1, 2),
            Literal::Integer(Token::new(1, Span::new_at("1", 0, 1, 1)))
        ));

        assert_eq!(binary(input), Err(Error::Error(Context::Code(input, ErrorKind::Tag))));
        assert_eq!(integer(input), output);
    }

    #[test]
    fn case_invalid_binary_not_in_base() {
        let input  = Span::new("0b120");
        let output = Ok((
            Span::new_at("20", 3, 1, 4),
            Literal::Integer(Token::new(1i64, Span::new_at("1", 2, 1, 3)))
        ));

        assert_eq!(binary(input),  output);
        assert_eq!(integer(input), output);
    }

    #[test]
    fn case_hexadecimal() {
        let input  = Span::new("0x2a\n");
        let output = Ok((
            Span::new_at("\n", 4, 1, 5),
            Literal::Integer(Token::new(42i64, Span::new_at("2a", 2, 1, 3)))
        ));

        assert_eq!(hexadecimal(input), output);
        assert_eq!(integer(input), output);
    }

    #[test]
    fn case_hexadecimal_uppercase_alpha() {
        let input  = Span::new("0x2A\n");
        let output = Ok((
            Span::new_at("\n", 4, 1, 5),
            Literal::Integer(Token::new(42i64, Span::new_at("2A", 2, 1, 3)))
        ));

        assert_eq!(hexadecimal(input), output);
        assert_eq!(integer(input), output);
    }

    #[test]
    fn case_invalid_hexadecimal_no_number() {
        let input  = Span::new("0x\n");
        let output = Ok((
            Span::new_at("x\n", 1, 1, 2),
            Literal::Integer(Token::new(0, Span::new_at("0", 0, 1, 1)))
        ));

        assert_eq!(hexadecimal(input), Err(Error::Error(Context::Code(Span::new_at("\n", 2, 1, 3),
                                                                      ErrorKind::HexDigit))));
        assert_eq!(integer(input), output);
    }

    #[test]
    fn case_invalid_hexadecimal_not_in_base() {
        let input  = Span::new("0xg");
        let output = Ok((
            Span::new_at("xg", 1, 1, 2),
            Literal::Integer(Token::new(0, Span::new_at("0", 0, 1, 1)))
        ));

        assert_eq!(hexadecimal(input), Err(Error::Error(Context::Code(Span::new_at("g", 2, 1, 3),
                                                                      ErrorKind::HexDigit))));
        assert_eq!(integer(input), output);
    }

    #[test]
    fn case_hexadecimal_maximum_integer_value() {
        let input  = Span::new("0x7fffffffffffffff\n");
        let output = Ok((
            Span::new_at("\n", 18, 1, 19),
            Literal::Integer(Token::new(::std::i64::MAX, Span::new_at("7fffffffffffffff", 2, 1, 3)))
        ));

        assert_eq!(hexadecimal(input), output);
        assert_eq!(integer(input), output);
    }

    #[test]
    fn case_invalid_hexadecimal_overflow() {
        let input  = Span::new("0x8000000000000000\n");
        let output = Ok((
            Span::new_at("x8000000000000000\n", 1, 1, 2),
            Literal::Integer(Token::new(0, Span::new_at("0", 0, 1, 1)))
        ));

        assert_eq!(hexadecimal(input), Err(Error::Error(Context::Code(input, ErrorKind::MapRes))));
        assert_eq!(integer(input), output);
    }

    #[test]
    fn case_octal() {
        let input  = Span::new("052\n");
        let output = Ok((
            Span::new_at("\n", 3, 1, 4),
            Literal::Integer(Token::new(42i64, Span::new_at("52", 1, 1, 2)))
        ));

        assert_eq!(octal(input), output);
        assert_eq!(integer(input), output);
    }

    #[test]
    fn case_octal_zero() {
        let input  = Span::new("0\n");
        let output = Ok((
            Span::new_at("\n", 1, 1, 2),
            Literal::Integer(Token::new(0i64, Span::new_at("0", 0, 1, 1)))
        ));

        assert_eq!(octal(input), Err(Error::Error(Context::Code(Span::new_at("\n", 1, 1, 2),
                                                                ErrorKind::OctDigit))));
        assert_eq!(integer(input), output);
    }

    #[test]
    fn case_octal_maximum_integer_value() {
        let input  = Span::new("0777777777777777777777\n");
        let output = Ok((
            Span::new_at("\n", 22, 1, 23),
            Literal::Integer(Token::new(::std::i64::MAX, Span::new_at("777777777777777777777", 1, 1, 2)))
        ));

        assert_eq!(octal(input), output);
        assert_eq!(integer(input), output);
    }

    #[test]
    fn case_invalid_octal_overflow() {
        let input  = Span::new("01000000000000000000000\n");
        let output = Err(Error::Error(Context::Code(input, ErrorKind::Alt)));

        assert_eq!(octal(input), Err(Error::Error(Context::Code(input, ErrorKind::MapRes))));
        assert_eq!(integer(input), output);
    }

    #[test]
    fn case_invalid_octal_not_starting_by_zero() {
        let input  = Span::new("7\n");
        let output = Ok((
            Span::new_at("\n", 1, 1, 2),
            Literal::Integer(Token::new(7i64, Span::new_at("7", 0, 1, 1)))
        ));

        assert_eq!(octal(input), Err(Error::Error(Context::Code(input, ErrorKind::Tag))));
        assert_eq!(integer(input), output);
    }

    #[test]
    fn case_invalid_octal_not_in_base() {
        let input  = Span::new("8\n");
        let output = Ok((
            Span::new_at("\n", 1, 1, 2),
            Literal::Integer(Token::new(8i64, Span::new_at("8", 0, 1, 1)))
        ));

        assert_eq!(octal(input), Err(Error::Error(Context::Code(input, ErrorKind::Tag))));
        assert_eq!(integer(input), output);
    }

    #[test]
    fn case_decimal_one_digit() {
        let input  = Span::new("7\n");
        let output = Ok((
            Span::new_at("\n", 1, 1, 2),
            Literal::Integer(Token::new(7i64, Span::new_at("7", 0, 1, 1)))
        ));

        assert_eq!(decimal(input), output);
        assert_eq!(integer(input), output);
    }

    #[test]
    fn case_decimal_many_digits() {
        let input  = Span::new("42\n");
        let output = Ok((
            Span::new_at("\n", 2, 1, 3),
            Literal::Integer(Token::new(42i64, Span::new_at("42", 0, 1, 1)))
        ));

        assert_eq!(decimal(input), output);
        assert_eq!(integer(input), output);
    }

    #[test]
    fn case_decimal_zero() {
        let input  = Span::new("0\n");
        let output = Ok((
            Span::new_at("\n", 1, 1, 2),
            Literal::Integer(Token::new(0i64, Span::new_at("0", 0, 1, 1)))
        ));

        assert_eq!(decimal(input), output);
        assert_eq!(integer(input), output);
    }

    #[test]
    fn case_decimal_plus() {
        let input  = Span::new("42+");
        let output = Ok((
            Span::new_at("+", 2, 1, 3),
            Literal::Integer(Token::new(42i64, Span::new("42")))
        ));

        assert_eq!(decimal(input), output);
        assert_eq!(integer(input), output);
    }

    #[test]
    fn case_decimal_maximum_integer_value() {
        let input  = Span::new("9223372036854775807\n");
        let output = Ok((
            Span::new_at("\n", 19, 1, 20),
            Literal::Integer(Token::new(::std::i64::MAX, Span::new_at("9223372036854775807", 0, 1, 1)))
        ));

        assert_eq!(decimal(input), output);
        assert_eq!(integer(input), output);
    }

    #[test]
    fn case_decimal_overflow_to_real() {
        let input  = Span::new("9223372036854775808\n");
        let output = Ok((
            Span::new_at("\n", 19, 1, 20),
            Literal::Real(Token::new(9223372036854775808f64, Span::new_at("9223372036854775808", 0, 1, 1)))
        ));

        assert_eq!(decimal(input), output);
        assert_eq!(integer(input), output);
    }

    #[test]
    fn case_decimal_maximum_real_value() {
        let input  = Span::new("179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\n");
        let output = Ok((
            Span::new_at("\n", 309, 1, 310),
            Literal::Real(Token::new(::std::f64::MAX, Span::new_at("179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000", 0, 1, 1)))
        ));

        assert_eq!(decimal(input), output);
        assert_eq!(integer(input), output);
    }

    #[test]
    fn case_invalid_decimal_overflow_to_infinity() {
        let input  = Span::new("1797693134862315700000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\n");
        let output = Ok((
            Span::new_at("\n", 310, 1, 311),
            Literal::Real(Token::new(::std::f64::INFINITY, Span::new_at("1797693134862315700000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000", 0, 1, 1)))
        ));

        assert_eq!(decimal(input), output);
        assert_eq!(integer(input), output);
    }

    #[test]
    fn case_real() {
        let input  = Span::new("123.456e+78\n");
        let output = Ok((
            Span::new_at("\n", 11, 1, 12),
            Literal::Real(Token::new(123.456e78f64, Span::new_at("123.456e+78", 0, 1, 1)))
        ));

        assert_eq!(real(input), output);
    }

    #[test]
    fn case_real_only_with_rational_and_fractional_part() {
        let input  = Span::new("0.456\n");
        let output = Ok((
            Span::new_at("\n", 5, 1, 6),
            Literal::Real(Token::new(0.456f64, Span::new_at("0.456", 0, 1, 1)))
        ));

        assert_eq!(real(input), output);
    }

    #[test]
    fn case_real_only_with_rational_part() {
        let input  = Span::new("123.\n");
        let output = Ok((
            Span::new_at("\n", 4, 1, 5),
            Literal::Real(Token::new(123.0f64, Span::new_at("123.", 0, 1, 1)))
        ));

        assert_eq!(real(input), output);
    }

    #[test]
    fn case_real_only_with_fractional_part() {
        let input  = Span::new(".456\n");
        let output = Ok((
            Span::new_at("\n", 4, 1, 5),
            Literal::Real(Token::new(0.456f64, Span::new_at(".456", 0, 1, 1)))
        ));

        assert_eq!(real(input), output);
    }

    #[test]
    fn case_real_only_with_rational_and_exponent_part_with_lowercase_e() {
        let input  = Span::new("123.e78\n");
        let output = Ok((
            Span::new_at("\n", 7, 1, 8),
            Literal::Real(Token::new(123e78f64, Span::new_at("123.e78", 0, 1, 1)))
        ));

        assert_eq!(real(input), output);
    }

    #[test]
    fn case_real_only_with_integer_rational_and_exponent_part() {
        let input  = Span::new("123e78\n");
        let output = Ok((
            Span::new_at("\n", 6, 1, 7),
            Literal::Real(Token::new(123e78f64, Span::new_at("123e78", 0, 1, 1)))
        ));

        assert_eq!(real(input), output);
    }

    #[test]
    fn case_real_only_with_rational_and_exponent_part_with_uppercase_e() {
        let input  = Span::new("123.E78\n");
        let output = Ok((
            Span::new_at("\n", 7, 1, 8),
            Literal::Real(Token::new(123e78f64, Span::new_at("123.E78", 0, 1, 1)))
        ));

        assert_eq!(real(input), output);
    }

    #[test]
    fn case_real_only_with_rational_and_unsigned_exponent_part() {
        let input  = Span::new("123.e78\n");
        let output = Ok((
            Span::new_at("\n", 7, 1, 8),
            Literal::Real(Token::new(123e78f64, Span::new_at("123.e78", 0, 1, 1)))
        ));

        assert_eq!(real(input), output);
    }

    #[test]
    fn case_real_only_with_rational_and_positive_exponent_part() {
        let input  = Span::new("123.e+78\n");
        let output = Ok((
            Span::new_at("\n", 8, 1, 9),
            Literal::Real(Token::new(123e78f64, Span::new_at("123.e+78", 0, 1, 1)))
        ));

        assert_eq!(real(input), output);
    }

    #[test]
    fn case_real_only_with_rational_and_negative_exponent_part() {
        let input  = Span::new("123.e-78\n");
        let output = Ok((
            Span::new_at("\n", 8, 1, 9),
            Literal::Real(Token::new(123e-78f64, Span::new_at("123.e-78", 0, 1, 1)))
        ));

        assert_eq!(real(input), output);
    }

    #[test]
    fn case_real_only_with_rational_and_negative_zero_exponent_part() {
        let input  = Span::new("123.e-0\n");
        let output = Ok((
            Span::new_at("\n", 7, 1, 8),
            Literal::Real(Token::new(123f64, Span::new_at("123.e-0", 0, 1, 1)))
        ));

        assert_eq!(real(input), output);
    }

    #[test]
    fn case_real_missing_exponent_part() {
        let input  = Span::new(".7e\n");
        let output = Ok((
            Span::new_at("e\n", 2, 1, 3),
            Literal::Real(Token::new(0.7f64, Span::new(".7")))
        ));

        assert_eq!(real(input), output);
    }

    #[test]
    fn case_invalid_real_only_the_dot() {
        let input = Span::new(".\n");

        assert_eq!(real(input), Err(Error::Error(Context::Code(input, ErrorKind::Alt))));
    }

    #[test]
    fn case_boolean_true() {
        let input  = Span::new("true\n");
        let output = Ok((
            Span::new_at("\n", 4, 1, 5),
            Literal::Boolean(Token::new(true, Span::new_at("true", 0, 1, 1)))
        ));

        assert_eq!(boolean(input), output);
    }

    #[test]
    fn case_boolean_false() {
        let input  = Span::new("false\n");
        let output = Ok((
            Span::new_at("\n", 5, 1, 6),
            Literal::Boolean(Token::new(false, Span::new_at("false", 0, 1, 1)))
        ));

        assert_eq!(boolean(input), output);
    }
}