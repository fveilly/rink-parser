use ast::ast::{
    Literal,
    Token
};

use span::Span;
use internal::ErrorKindExtension;

use std::result::Result as StdResult;
use std::ops::{Range, RangeFrom, RangeTo};

use std::num::{
    ParseFloatError,
    ParseIntError
};

use nom::{
    InputIter,
    InputLength,
    Slice,
    IResult,
    AsChar,
    AtEof,
    hex_digit
};

named_attr!(
    #[doc="
        Recognize an integer.
        An integer is either a binary, a decimal, an hexadecimal or an octal representation.
    "],
    pub integer<Span, Literal>,
    alt_complete!(
        binary
        | hexadecimal
      /*| decimal
      | octal*/
    )
);

#[inline]
pub fn is_binary_digit(chr: char) -> bool {
    chr == '0' || chr == '1'
}

/// Recognizes one or more binary numerical characters
pub fn binary_digit(input: Span) -> IResult<Span, Span>
{
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

named_attr!(
    #[doc="
        Recognize an integer with the binary notation.
        # Examples
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
        # Examples
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

#[cfg(test)]
mod tests {
    use super::{
        integer,
        binary,
        hexadecimal
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

        assert_eq!(binary(input), Err(Error::Error(Context::Code(input, ErrorKind::MapRes))));
        assert_eq!(integer(input),  Err(Error::Error(Context::Code(input, ErrorKind::Alt))));
    }

    #[test]
    fn case_invalid_binary_no_number() {
        let input  = Span::new("0b\n");
        let output = Span::new_at("\n", 2, 1, 3);

        assert_eq!(binary(input), Err(Error::Error(Context::Code(output, ErrorKind::Custom(ErrorKindExtension::BinaryDigit as u32)))));
        assert_eq!(integer(input), Err(Error::Error(Context::Code(input, ErrorKind::Alt))));
    }

    #[test]
    fn case_invalid_binary_not_starting_by_zero_b() {
        let input  = Span::new("1\n");

        assert_eq!(binary(input), Err(Error::Error(Context::Code(input, ErrorKind::Tag))));
        assert_eq!(integer(input), Err(Error::Error(Context::Code(input, ErrorKind::Alt))));
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

        assert_eq!(hexadecimal(input), Err(Error::Error(Context::Code(Span::new_at("\n", 2, 1, 3),
                                                                      ErrorKind::HexDigit))));
        assert_eq!(integer(input), Err(Error::Error(Context::Code(input, ErrorKind::Alt))));
    }

    #[test]
    fn case_invalid_hexadecimal_not_in_base() {
        let input  = Span::new("0xg");

        assert_eq!(hexadecimal(input), Err(Error::Error(Context::Code(Span::new_at("g", 2, 1, 3),
                                                                      ErrorKind::HexDigit))));
        assert_eq!(integer(input), Err(Error::Error(Context::Code(input, ErrorKind::Alt))));
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

        assert_eq!(hexadecimal(input), Err(Error::Error(Context::Code(input, ErrorKind::MapRes))));
        assert_eq!(integer(input), Err(Error::Error(Context::Code(input, ErrorKind::Alt))));
    }
}