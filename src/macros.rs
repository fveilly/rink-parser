/// `skip_many0!(I -> IResult<I,O>) => I -> IResult<I,O>`
/// Applies the parser 0 or more times and discard the results
///
/// The embedded parser may return Incomplete
#[macro_export]
macro_rules! skip_many0(
  ($i:expr, $submac:ident!( $($args:tt)* )) => (
    {
      use ::std::result::Result::*;
      use nom::{Err,AtEof};

      let ret;
      let mut input = $i.clone();

      loop {
        let input_ = input.clone();
        match $submac!(input_, $($args)*) {
          Err(Err::Error(_))      => {
            ret = Ok(input);
            break;
          },
          Err(e) => {
            ret = Err(e);
            break;
          },
          Ok(i)              => {
            // Loop trip must always consume (otherwise infinite loops)
            if i == input {

              if i.at_eof() {
                ret = Ok(input);
              } else {
                ret = Err(Err::Error(error_position!(input, nom::ErrorKind::Many0)));
              }
              break;
            }

            input = i;
          }
        }
      }

      ret
    }
  );
  ($i:expr, $f:expr) => (
    skip_many0!($i, call!($f));
  );
);

/// `first!(I -> Result<I, O>) => I -> Result<I, O>`
/// is applying the `skip` rule before the first argument; it allows to skip
/// tokens.
#[macro_export]
macro_rules! first(
    ($input:expr, $submacro:ident!($($arguments:tt)*)) => (
        {
            preceded!(
                $input,
                call!($crate::rules::skip::skip),
                $submacro!($($arguments)*)
            )
        }
    );

    ($input:expr, $f:expr) => (
        first!($input, call!($f));
    );
);

/// `take_until_eof_and_consume!(tag) => &[T] -> IResult<&[T], &[T]>`
/// generates a parser consuming bytes until the specified byte sequence is found, and consumes it
///
/// Equivalent to nom take_until_and_consume!() macro but does not return Incomplete if eof is reached.
#[macro_export]
macro_rules! take_until_eof_and_consume (
  ($i:expr, $substr:expr) => (
    {
      use ::std::result::Result::*;
      use ::std::option::Option::*;
      use nom::{Err,Needed,IResult};
      use nom::InputLength;
      use nom::AtEof;
      use nom::FindSubstring;
      use nom::Slice;

      let input = $i;

      let res: IResult<_,_> = match input.find_substring($substr) {
        None => {
            Ok(($i.slice($i.input_len()..), $i))
        },
        Some(index) => {
          Ok(($i.slice(index+$substr.input_len()..), $i.slice(0..index)))
        },
      };
      res
    }
  );
);

#[macro_export]
macro_rules! take_until_endline_and_consume (
  ($i:expr,) => (
    {
      use ::std::result::Result::*;
      use ::std::option::Option::*;
      use nom::{Err,Needed,IResult};
      use nom::InputLength;
      use nom::AtEof;
      use nom::FindSubstring;
      use nom::Slice;

      let input = $i;
      let mut delimiter_size = 1;

      let res: IResult<_,_> = match input.find_substring('\n') {
        None => {
            Ok(($i.slice($i.input_len()..), $i))
        },
        Some(mut index) => {
            if index != 0 {
                if $i.as_slice().as_bytes()[index - 1] as char == '\r' {
                    index -= 1;
                    delimiter_size += 1;
                }
            }

          Ok(($i.slice(index+delimiter_size..), $i.slice(0..index)))
        },
      };
      res
    }
  );
);

#[cfg(test)]
mod tests {
    use super::super::span::Span;

    named!(
        test_take_until_endline_and_consume<Span, Span>,
        take_until_endline_and_consume!()
    );

    #[test]
    fn case_take_until_endline_and_consume_empty() {
        let input  = Span::new("");
        let output = Ok((Span::new_at("", 0, 1, 1), Span::new_at("", 0, 1, 1)));

        assert_eq!(test_take_until_endline_and_consume(input), output);
    }

    #[test]
    fn case_take_until_endline_and_consume_newline() {
        let input  = Span::new("\n");
        let output = Ok((Span::new_at("", 1, 2, 1), Span::new_at("", 0, 1, 1)));

        assert_eq!(test_take_until_endline_and_consume(input), output);
    }

    #[test]
    fn case_take_until_endline_and_consume_carriage_return() {
        let input  = Span::new("\r\n");
        let output = Ok((Span::new_at("", 2, 2, 1), Span::new_at("", 0, 1, 1)));

        assert_eq!(test_take_until_endline_and_consume(input), output);
    }

    #[test]
    fn case_take_until_endline_and_consume_with_feed() {
        let input  = Span::new("foobar \nbazqux");
        let output = Ok((Span::new_at("bazqux", 8, 2, 1), Span::new_at("foobar ", 0, 1, 1)));

        assert_eq!(test_take_until_endline_and_consume(input), output);
    }

    #[test]
    fn case_take_until_endline_and_consume_with_carriage_return_feed() {
        let input  = Span::new("foobar \r\nbazqux");
        let output = Ok((Span::new_at("bazqux", 9, 2, 1), Span::new_at("foobar ", 0, 1, 1)));

        assert_eq!(test_take_until_endline_and_consume(input), output);
    }

    #[test]
    fn case_take_until_endline_and_consume_missing_endline() {
        let input  = Span::new("foobar");
        let output = Ok((Span::new_at("", 6, 1, 7), Span::new_at("foobar", 0, 1, 1)));

        assert_eq!(test_take_until_endline_and_consume(input), output);
    }

    #[test]
    fn case_take_until_endline_and_consume_utf8() {
        let input  = Span::new("ロ－\n");
        let output = Ok((Span::new_at("", 7, 2, 1), Span::new_at("ロ－", 0, 1, 1)));

        assert_eq!(test_take_until_endline_and_consume(input), output);
    }
}