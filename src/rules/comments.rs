use super::super::tokens;
use super::super::span::Span;

named_attr!(
    #[doc="
        Recognize all kind of comments.
        A comment can be a single line (`//`) or a delimited block (`/* … */`).
    "],
    pub comment<Span, Span>,
    alt!(
        comment_delimited
        | comment_single_line
    )
);

named!(
    comment_delimited<Span, Span>,
    preceded!(
        tag!(tokens::BLOCK_COMMENT_OPEN),
        complete!(take_until_and_consume_s!(tokens::BLOCK_COMMENT_CLOSE))
    )
);

named!(
    comment_single_line<Span, Span>,
    preceded!(
        tag!(tokens::INLINE_COMMENT),
        incomplete!(take_until_endline_and_consume!())
    )
);

#[cfg(test)]
mod tests {
    use super::{
        comment,
        comment_delimited,
        comment_single_line
    };
    use super::super::super::internal::{
        Context,
        Error,
        ErrorKind,
    };
    use super::super::super::span::Span;
    use nom::Needed;

    #[test]
    fn case_comment_single_line_double_slash_empty() {
        let input  = Span::new("//");
        let output = Ok((Span::new_at("", 2, 1, 3), Span::new_at("", 2, 1, 3)));

        assert_eq!(comment_single_line(input), output);
        assert_eq!(comment(input), output);
    }

    #[test]
    fn case_comment_single_line_double_slash_with_feed() {
        let input  = Span::new("// foobar\nbazqux");
        let output = Ok((Span::new_at("bazqux", 10, 2, 1), Span::new_at(" foobar", 2, 1, 3)));

        assert_eq!(comment_single_line(input), output);
        assert_eq!(comment(input), output);
    }

    #[test]
    fn case_comment_single_line_double_slash_with_double_endline() {
        let input  = Span::new("// foobar\n\nbazqux");
        let output = Ok((Span::new_at("\nbazqux", 10, 2, 1), Span::new_at(" foobar", 2, 1, 3)));

        assert_eq!(comment_single_line(input), output);
        assert_eq!(comment(input), output);
    }

    #[test]
    fn case_comment_single_line_double_slash_with_carriage_return_feed() {
        let input  = Span::new("// foobar\r\nbazqux");
        let output = Ok((Span::new_at("bazqux", 11, 2, 1), Span::new_at(" foobar", 2, 1, 3)));

        assert_eq!(comment_single_line(input), output);
        assert_eq!(comment(input), output);
    }

    #[test]
    fn case_comment_single_line_double_slash_without_ending() {
        let input  = Span::new("// foobar");
        let output = Ok((Span::new_at("", 9, 1, 10), Span::new_at(" foobar", 2, 1, 3)));

        assert_eq!(comment_single_line(input), output);
        assert_eq!(comment(input), output);
    }

    #[test]
    fn case_comment_single_line_double_slash_embedded() {
        let input  = Span::new("//foo//bar");
        let output = Ok((Span::new_at("", 10, 1, 11), Span::new_at("foo//bar", 2, 1, 3)));

        assert_eq!(comment_single_line(input), output);
        assert_eq!(comment(input), output);
    }

    #[test]
    fn case_comment_delimited_empty() {
        let input  = Span::new("/**/xyz");
        let output = Ok((Span::new_at("xyz", 4, 1, 5), Span::new_at("", 2, 1, 3)));

        assert_eq!(comment_delimited(input), output);
        assert_eq!(comment(input), output);
    }

    #[test]
    fn case_comment_delimited_almost_nested() {
        let input  = Span::new("/****/xyz");
        let output = Ok((Span::new_at("xyz", 6, 1, 7), Span::new_at("**", 2, 1, 3)));

        assert_eq!(comment_delimited(input), output);
        assert_eq!(comment(input), output);
    }

    #[test]
    fn case_comment_delimited() {
        let input  = Span::new("/* foo bar\nbaz\r\nqux // hello,\n /*world!*/xyz */");
        let output = Ok((Span::new_at("xyz */", 41, 4, 12), Span::new_at(" foo bar\nbaz\r\nqux // hello,\n /*world!", 2, 1, 3)));

        assert_eq!(comment_delimited(input), output);
        assert_eq!(comment(input), output);
    }

    #[test]
    fn case_invalid_comment_delimited_not_closed() {
        let input = Span::new("/*foobar");

        assert_eq!(comment_delimited(input), Err(Error::Error(Context::Code(Span::new_at("foobar", 2, 1, 3), ErrorKind::Complete))));
        assert_eq!(comment(input), Err(Error::Error(Context::Code(Span::new_at("/*foobar", 0, 1, 1), ErrorKind::Alt))));
    }

}