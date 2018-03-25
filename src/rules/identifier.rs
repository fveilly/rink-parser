use span::Span;
use internal::ErrorKindExtension;

use nom::{
    IResult,
    digit,
    alphanumeric
};

#[inline]
pub fn is_identifier(chr: char) -> bool {
    chr == '_' || chr.is_alphanumeric()
}

/// Recognizes an identifier. An identifier must follow these rules:
/// - Only Alphabets, Digits and Underscores are permitted.
/// - Identifier name cannot start with a digit.
/// - Key words cannot be used as a name.
/// - Upper case and lower case letters are distinct.
/// - Special Characters are not allowed
pub fn identifier(input: Span) -> IResult<Span, Span> {
    use nom::{
        Err,
        ErrorKind,
        Needed,
        InputLength,
        Slice,
        AtEof,
        InputIter
    };

    match input.position(|item| !is_identifier(item)) {
        Some(0) => Err(Err::Error(error_position!(input, ErrorKind::Custom(ErrorKindExtension::Identifier as u32)))),
        Some(n) => {
            match input.as_slice().chars().next().and_then(|chr| Some(chr.is_numeric())) {
                Some(false) => { Ok((input.slice(n..), input.slice(..n))) },
                _ => { Err(Err::Error(error_position!(input, ErrorKind::Custom(ErrorKindExtension::Identifier as u32)))) }
            }
        },
        None => {
            if input.at_eof() {
                if input.input_len() > 0 {
                    Ok((input.slice(input.input_len()..), input))
                } else {
                    Err(Err::Error(error_position!(input, ErrorKind::Custom(ErrorKindExtension::Identifier as u32))))
                }
            } else {
                Err(Err::Incomplete(Needed::Size(1)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::identifier;

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
    fn case_identifier() {
        let input = Span::new("Name\n");
        let output = Ok((
            Span::new_at("\n", 4, 1, 5),
            Span::new_at("Name", 0, 1, 1)
        ));

        assert_eq!(identifier(input), output);
    }

    #[test]
    fn case_identifier_lowercase() {
        let input = Span::new("name\n");
        let output = Ok((
            Span::new_at("\n", 4, 1, 5),
            Span::new_at("name", 0, 1, 1)
        ));

        assert_eq!(identifier(input), output);
    }

    #[test]
    fn case_identifier_uppercase() {
        let input = Span::new("NAME\n");
        let output = Ok((
            Span::new_at("\n", 4, 1, 5),
            Span::new_at("NAME", 0, 1, 1)
        ));

        assert_eq!(identifier(input), output);
    }

    #[test]
    fn case_identifier_underscore() {
        let input = Span::new("name_1\n");
        let output = Ok((
            Span::new_at("\n", 6, 1, 7),
            Span::new_at("name_1", 0, 1, 1)
        ));

        assert_eq!(identifier(input), output);
    }

    #[test]
    fn case_identifier_multiple_underscore() {
        let input = Span::new("sum_of_the_numbers\n");
        let output = Ok((
            Span::new_at("\n", 18, 1, 19),
            Span::new_at("sum_of_the_numbers", 0, 1, 1)
        ));

        assert_eq!(identifier(input), output);
    }

    #[test]
    fn case_identifier_1() {
        let input = Span::new("x\n");
        let output = Ok((
            Span::new_at("\n", 1, 1, 2),
            Span::new_at("x", 0, 1, 1)
        ));

        assert_eq!(identifier(input), output);
    }

    #[test]
    fn case_underscore_first() {
        let input = Span::new("_SUM\n");
        let output = Ok((
            Span::new_at("\n", 4, 1, 5),
            Span::new_at("_SUM", 0, 1, 1)
        ));

        assert_eq!(identifier(input), output);
    }

    #[test]
    fn case_identifier_special_char() {
        let input = Span::new("name.data\n");
        let output = Ok((
            Span::new_at(".data\n", 4, 1, 5),
            Span::new_at("name", 0, 1, 1)
        ));

        assert_eq!(identifier(input), output);
    }

    #[test]
    fn case_invalid_identifier_empty() {
        let input = Span::new("\n");

        assert_eq!(identifier(input), Err(Error::Error(Context::Code( Span::new_at("\n", 0, 1, 1),
                                                                      ErrorKind::Custom(ErrorKindExtension::Identifier as u32)))));
    }

    #[test]
    fn case_invalid_identifier_start_with_digit() {
        let input = Span::new("5name\n");

        assert_eq!(identifier(input), Err(Error::Error(Context::Code( Span::new_at("5name\n", 0, 1, 1),
                                                                      ErrorKind::Custom(ErrorKindExtension::Identifier as u32)))));
    }

    #[test]
    fn case_invalid_identifier_1_digit() {
        let input = Span::new("5\n");

        assert_eq!(identifier(input), Err(Error::Error(Context::Code( Span::new_at("5\n", 0, 1, 1),
                                                                      ErrorKind::Custom(ErrorKindExtension::Identifier as u32)))));
    }

    #[test]
    fn case_identifier_special_char_first() {
        let input = Span::new("$name\n");

        assert_eq!(identifier(input), Err(Error::Error(Context::Code( Span::new_at("$name\n", 0, 1, 1),
                                                                      ErrorKind::Custom(ErrorKindExtension::Identifier as u32)))));
    }
}