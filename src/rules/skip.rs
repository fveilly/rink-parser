use super::comments::comment;
use super::super::span::Span;

named_attr!(
    #[doc="
        Recognize all tokens to skip.
        A skip token is a token that is not relevant for the understanding of
        the language. It is present for cosmetic reasons only.
    "],
    pub skip<Span, Vec<Span>>,
    skip_many0!(
        alt!(
            comment
            | whitespace
        )
    )
);


#[cfg(test)]
mod tests {
    use super::skip;
    use super::super::super::tokens::Span;

    #[test]
    fn case_skip_comment() {
        let input  = Span::new("/* foo */hello");
        let output = Ok(Span::new_at("hello", 9, 1, 10));

        assert_eq!(skip(input), output);
    }

    #[test]
    fn case_skip_whitespace() {
        let input  = Span::new("  \nhello");
        let output = Ok(Span::new_at("hello", 3, 2, 1));

        assert_eq!(skip(input), output);
    }

    #[test]
    fn case_skip_comment_whitespace() {
        let input  = Span::new("/* foo */  \nhello");
        let output = Ok(Span::new_at("hello", 12, 2, 1));

        assert_eq!(skip(input), output);
    }
}