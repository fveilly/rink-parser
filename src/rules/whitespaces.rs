use super::super::span::Span;

named_attr!(
    #[doc="
        Recognize all whitespaces.
    "],
    pub whitespace<Span, Span>,
    is_a!(" \t\n\r")
);

#[cfg(test)]
mod tests {
    use super::super::super::internal::{
        Context,
        Error,
        ErrorKind
    };
    use super::super::super::span::Span;
    use super::whitespace;

    #[test]
    fn case_whitespace_space() {
        let input  = Span::new("   ");
        let output = Ok((Span::new_at("", 3, 1, 4), input));

        assert_eq!(whitespace(input), output);
    }

    #[test]
    fn case_whitespace_horizontal_tabulation() {
        let input  = Span::new("\t\t\t");
        let output = Ok((Span::new_at("", 3, 1, 4), input));

        assert_eq!(whitespace(input), output);
    }

    #[test]
    fn case_whitespace_carriage_return_line_feed() {
        let input  = Span::new("\r\n\r\n\r\n");
        let output = Ok((Span::new_at("", 6, 4, 1), input));

        assert_eq!(whitespace(input), output);
    }

    #[test]
    fn case_whitespace_carriage_return() {
        let input  = Span::new("\r\r\r");
        let output = Ok((Span::new_at("", 3, 1, 4), input));

        assert_eq!(whitespace(input), output);
    }

    #[test]
    fn case_whitespace_line_feed() {
        let input  = Span::new("\n\n\n");
        let output = Ok((Span::new_at("", 3, 4, 1), input));

        assert_eq!(whitespace(input), output);
    }

    #[test]
    fn case_whitespace_mixed() {
        let input  = Span::new("\n \n \r\t  \t\r\n\t \t\t");
        let output = Ok((Span::new_at("", 15, 4, 5), input));

        assert_eq!(whitespace(input), output);
    }

    #[test]
    fn case_whitespace_with_a_tail() {
        let input  = Span::new("\n \n \r\t  \t\r\n\t \t\tabc ");
        let output = Ok((Span::new_at("abc ", 15, 4, 5), Span::new("\n \n \r\t  \t\r\n\t \t\t")));

        assert_eq!(whitespace(input), output);
    }

    #[test]
    fn case_whitespace_too_short() {
        let input  = Span::new("");
        let output = Ok((Span::new_at("", 0, 1, 1), input));

        assert_eq!(whitespace(input), output);
    }

    #[test]
    fn case_invalid_whitespace_not_a_valid_character() {
        let input  = Span::new("abc\n \t");
        let output = Err(Error::Error(Context::Code(input, ErrorKind::IsA)));

        assert_eq!(whitespace(input), output);
    }
}