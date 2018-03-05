use super::super::span::Span;

named_attr!(
    #[doc="
        Recognize all whitespaces.
    "],
    pub whitespace<Span, Span>,
    incomplete!(is_a!(" \t"))
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
    fn case_whitespace_mixed() {
        let input  = Span::new("    \t  \t\t \t\t");
        let output = Ok((Span::new_at("", 12, 1, 13), input));

        assert_eq!(whitespace(input), output);
    }

    #[test]
    fn case_whitespace_with_a_tail() {
        let input  = Span::new("    \t  \t\t \t\tabc ");
        let output = Ok((Span::new_at("abc ", 12, 1, 13), Span::new("    \t  \t\t \t\t")));

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