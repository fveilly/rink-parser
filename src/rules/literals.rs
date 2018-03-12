use super::number::number;

use ast::ast::Literal;
use span::Span;

named_attr!(
    #[doc="
        Recognize all kind of literals.
        A literal is either a number or a string.
    "],
    pub literal<Span, Literal>,
    alt!(
        number
      //| string
    )
);

#[cfg(test)]
mod tests {
    use super::literal;

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
    fn case_literal_boolean() {
        let input  = Span::new("true\n");

        assert_eq!(literal(input), Ok((
            Span::new_at("\n", 4, 1, 5),
            Literal::Boolean(Token::new(true, Span::new_at("true", 0, 1, 1)))
        )));
    }

    #[test]
    fn case_literal_binary() {
        let input  = Span::new("0b10011001011011101\n");

        assert_eq!(literal(input), Ok((
            Span::new_at("\n", 19, 1, 20),
            Literal::Integer(Token::new(78557i64, Span::new_at("10011001011011101", 2, 1, 3)))
        )));
    }

    #[test]
    fn case_literal_hexadecimal() {
        let input  = Span::new("0x132DD\n");

        assert_eq!(literal(input), Ok((
            Span::new_at("\n", 7, 1, 8),
            Literal::Integer(Token::new(78557i64, Span::new_at("132DD", 2, 1, 3)))
        )));
    }

    #[test]
    fn case_literal_octal() {
        let input  = Span::new("0231335\n");

        assert_eq!(literal(input), Ok((
            Span::new_at("\n", 7, 1, 8),
            Literal::Integer(Token::new(78557i64, Span::new_at("231335", 1, 1, 2)))
        )));
    }

    #[test]
    fn case_literal_decimal() {
        let input  = Span::new("78557\n");

        assert_eq!(literal(input), Ok((
            Span::new_at("\n", 5, 1, 6),
            Literal::Integer(Token::new(78557i64, Span::new_at("78557", 0, 1, 1)))
        )));
    }

    #[test]
    fn case_literal_real() {
        let input  = Span::new("1.6180339887498948482\n");
        let value = ‎1.6180339887498948482f64;

        assert_eq!(literal(input), Ok((
            Span::new_at("\n", 21, 1, 22),
            Literal::Real(Token::new(value, Span::new_at("1.6180339887498948482", 0, 1, 1)))
        )));
    }
}