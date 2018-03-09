use nom::{
    AtEof,
    Compare,
    CompareResult,
    FindSubstring,
    InputIter,
    InputLength,
    InputTake,
    Offset,
    Slice
};

use std::str::Chars;
use std::str::CharIndices;
use std::cmp::Ordering;

use std::ops::{
    Range,
    RangeFrom,
    RangeFull,
    RangeTo
};

use memchr;

/// A span is a set of meta information about a token.
///
/// The `Span` structure can be used as an input of the nom parsers.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Span<'a> {
    /// The offset represents the position of the slice relatively to
    /// the input of the parser_old. It starts at offset 0.
    pub offset: usize,

    /// The line number of the slice relatively to the input of the
    /// parser_old. It starts at line 1.
    pub line: u32,

    /// The column number of the slice relatively to the input of the
    /// parser_old. It starts at column 1.
    pub column: u32,

    /// The fragment that is spanned.
    fragment: &'a str
}

impl<'a> Span<'a> {
    /// Create a span for a particular input with default `offset`,
    /// `line`, and `column` values.
    ///
    /// `offset` starts at 0, `line` starts at 1, and `column` starts at 1.
    pub fn new(input: &'a str) -> Self {
        Span {
            offset: 0,
            line  : 1,
            column: 1,
            fragment : input
        }
    }

    /// Create a span for a particular input at a particular offset, line, and column.
    pub fn new_at(input: &'a str, offset: usize, line: u32, column: u32) -> Self {
        Span {
            offset: offset,
            line  : line,
            column: column,
            fragment : input
        }
    }

    /// Create a blank span.
    #[inline]
    pub fn empty() -> Self {
        Self::new("")
    }

    /// Extract the entire slice of the span.
    #[inline]
    pub fn as_slice(&self) -> &'a str {
        self.fragment
    }
}

/// Implement `InputLength` from nom to be able to use the `Span`
/// structure as an input of the parsers.
///
/// This trait aims at computing the length of the input.
impl<'a> InputLength for Span<'a> {
    /// Compute the length of the slice in the span.
    #[inline]
    fn input_len(&self) -> usize {
        self.fragment.len()
    }
}

/// Implement `InputTake` from nom to be able to use the `Span`
/// structure as an input of the parsers.
impl<'a> InputTake for Span<'a> {
    /// Returns a slice of `count` bytes. panics if count > length
    #[inline]
    fn take(&self, count: usize) -> Self {
        self.slice(..count)
    }

    /// Split the stream at the `count` byte offset. panics if count > length
    #[inline]
    fn take_split(&self, count: usize) -> (Self, Self)
    {
        (self.slice(count..), self.slice(..count))
    }
}

/// Implement `AtEof` from nom to be able to use the `Span` structure
/// as an input of the parsers.
///
/// This trait aims at determining whether the current span is at the
/// end of the input.
impl<'a> AtEof for Span<'a> {
    #[inline]
    fn at_eof(&self) -> bool {
        self.fragment.at_eof()
    }
}

/// Implement `InputIter` from nom to be able to use the `Span`
/// structure as an input of the parsers.
///
/// This trait aims at iterating over the input.
impl<'a> InputIter for Span<'a> {
    /// Type of an element of the span' slice.
    type Item     = char;

    /// Type of a raw element of the span' slice.
    type RawItem  = char;

    /// Type of the enumerator iterator.
    type Iter     = CharIndices<'a>;

    /// Type of the iterator.
    type IterElem = Chars<'a>;

    /// Return an iterator that enumerates the byte offset and the
    /// element of the slice in the span.
    #[inline]
    fn iter_indices(&self) -> Self::Iter {
        self.fragment.char_indices()
    }

    /// Return an iterator over the elements of the slice in the span.
    #[inline]
    fn iter_elements(&self) -> Self::IterElem {
        self.fragment.chars()
    }

    /// Find the byte position of an element in the slice of the span.
    #[inline]
    fn position<P>(&self, predicate: P) -> Option<usize>
        where P: Fn(Self::RawItem) -> bool {
        for (o, c) in self.fragment.char_indices() {
            if predicate(c) {
                return Some(o);
            }
        }
        None
    }

    /// Get the byte offset from the element's position in the slice
    /// of the span.
    #[inline]
    fn slice_index(&self, count: usize) -> Option<usize> {
        let mut cnt = 0;
        for (index, _) in self.fragment.char_indices() {
            if cnt == count {
                return Some(index);
            }
            cnt += 1;
        }
        if cnt == count {
            return Some(self.fragment.len());
        }
        None
    }
}

/// Implement `FindSubstring` from nom to be able to use the `Span`
/// structure as an input of the parsers.
///
/// This traits aims at finding a substring in an input.
impl<'a, 'b> FindSubstring<&'b str> for Span<'a> {
    /// Find the position of a substring in the current span.
    #[inline]
    fn find_substring(&self, substring: &'b str) -> Option<usize> {
        if substring.is_empty() {
            return None;
        }

        self.fragment.find(substring)
    }
}

impl<'a> FindSubstring<char> for Span<'a> {
    /// Find the position of a char in the current span.
    #[inline]
    fn find_substring(&self, c: char) -> Option<usize> {
        self.fragment.find(c)
    }
}

/// Implement `Compare` from nom to be able to use the `Span`
/// structure as an input of the parsers.
///
/// This trait aims at comparing inputs.
impl<'a, 'b> Compare<&'b str> for Span<'a> {
    /// Compare self to another input for equality.
    #[inline]
    fn compare(&self, element: &'b str) -> CompareResult {
        self.fragment.compare(element)
    }

    /// Compare self to another input for equality independently of the case.
    #[inline]
    fn compare_no_case(&self, element: &'b str) -> CompareResult {
        self.fragment.compare_no_case(element)
    }
}

impl<'a> Compare<char> for Span<'a> {
    /// Compare self to another input for equality.
    fn compare(&self, c: char) -> CompareResult {
        match self.fragment.chars().nth(0) {
            Some(first_char) => {
                if first_char == c {
                    CompareResult::Ok
                } else {
                    CompareResult::Error
                }
            }
            None => { CompareResult::Error }
        }
    }

    /// Compare self to another input for equality independently of the case.
    fn compare_no_case(&self, c: char) -> CompareResult {
        match self.fragment.chars().nth(0) {
            Some(first_char) => {
                if first_char.to_lowercase().cmp(c.to_lowercase()) == Ordering::Equal {
                    CompareResult::Ok
                } else {
                    CompareResult::Error
                }
            }
            None => { CompareResult::Error }
        }
    }
}

macro_rules! impl_slice_for_range {
    ($range:ty) => (
        /// Implement a range from nom to be able to use the `Span`
        /// structure as an input of the parsers.
        ///
        /// This trait aims at slicing inputs based of a range.
        impl<'a> Slice<$range> for Span<'a> {
            /// Slice the span' slice based on a particular range.
            ///
            /// This is where new spans are computed.
            fn slice(&self, range: $range) -> Self {
                let next_fragment = self.fragment.slice(range);
                if next_fragment == self.fragment {
                    return *self;
                }

                let next_offset = self.fragment.offset(&next_fragment);
                if next_offset == 0 {
                    return Span {
                        line: self.line,
                        offset: self.offset,
                        column: self.column,
                        fragment: next_fragment
                    };
                }

                let consumed = self.fragment.slice(..next_offset);
                let consumed_as_bytes = consumed.as_bytes();
                let iter = memchr::Memchr::new(b'\n', consumed_as_bytes);
                let number_of_newlines = iter.count() as u32;

                let next_column =
                    if number_of_newlines == 0 {
                        self.column + next_offset as u32
                    } else {
                        match memchr::memrchr(b'\n', consumed_as_bytes) {
                            Some(last_newline_position) => {
                                (next_offset - last_newline_position) as u32
                            },

                            None => {
                                unreachable!();
                            }
                        }
                    };

                Span {
                    line: self.line + number_of_newlines,
                    offset: self.offset + next_offset,
                    column: next_column,
                    fragment: next_fragment
                }
            }
        }
    )
}

impl_slice_for_range!(Range<usize>);
impl_slice_for_range!(RangeTo<usize>);
impl_slice_for_range!(RangeFrom<usize>);
impl_slice_for_range!(RangeFull);

#[cfg(test)]
mod tests {
    use super::Span;
    use nom::{
        Compare,
        CompareResult,
        FindSubstring,
        InputIter,
        InputLength,
        Slice
    };

    #[test]
    fn case_span_new() {
        let input  = "foobar";
        let output = Span {
            offset: 0,
            line  : 1,
            column: 1,
            fragment : input
        };

        assert_eq!(Span::new(input), output);
    }

    #[test]
    fn case_span_new_at() {
        let input  = "foobar";
        let output = Span {
            offset: 1,
            line  : 2,
            column: 3,
            fragment : input
        };

        assert_eq!(Span::new_at(input, 1, 2, 3), output);
    }

    #[test]
    fn case_span_empty() {
        let output = Span {
            offset: 0,
            line  : 1,
            column: 1,
            fragment : ""
        };

        assert_eq!(Span::empty(), output);
    }

    #[test]
    fn case_span_as_slice() {
        let input  = Span::new("foobar");
        let output = "foobar";

        assert_eq!(input.as_slice(), output);
    }

    #[test]
    fn case_span_from() {
        let input  = "foobar";
        let output = Span::new(input);

        assert_eq!(Span::new(input), output);
    }

    #[test]
    fn case_span_length_zero() {
        let input  = Span::new("");
        let output = 0;

        assert_eq!(input.input_len(), output);
    }

    #[test]
    fn case_span_length_many() {
        let input  = Span::new("foobar");
        let output = 6;

        assert_eq!(input.input_len(), output);
    }

    #[test]
    fn case_span_empty_iterator_with_indices() {
        let input  = Span::new("");
        let output = 0;

        assert_eq!(input.iter_indices().count(), output);
    }

    #[test]
    fn case_span_iterator_with_indices() {
        let input  = Span::new("foobar");
        let output = vec![
            (0, 'f'),
            (1, 'o'),
            (2, 'o'),
            (3, 'b'),
            (4, 'a'),
            (5, 'r')
        ];

        let mut accumulator = Vec::new();

        for (index, item) in input.iter_indices() {
            accumulator.push((index, item));
        }

        assert_eq!(accumulator, output);
    }

    #[test]
    fn case_span_empty_iterator() {
        let input  = Span::new("");
        let output = 0;

        assert_eq!(input.iter_elements().count(), output);
    }

    #[test]
    fn case_span_iterator_with_elements() {
        let input  = Span::new("foobar");
        let output = vec!['f', 'o', 'o', 'b', 'a', 'r'];

        let mut accumulator = Vec::new();

        for item in input.iter_elements() {
            accumulator.push(item);
        }

        assert_eq!(accumulator, output);
    }

    #[test]
    fn case_span_empty_position() {
        let input  = Span::new("");
        let output = None;

        assert_eq!(input.position(|x| x == 'a'), output);
    }

    #[test]
    fn case_span_position() {
        let input  = Span::new("foobar");
        let output = Some(4);

        assert_eq!(input.position(|x| x == 'a'), output);
    }

    #[test]
    fn case_span_position_not_found() {
        let input  = Span::new("foobar");
        let output = None;

        assert_eq!(input.position(|x| x == 'z'), output);
    }

    #[test]
    fn case_span_empty_index() {
        let input  = Span::new("");
        let output = None;

        assert_eq!(input.slice_index(2), output);
    }

    #[test]
    fn case_span_index() {
        let input  = Span::new("foobar");
        let output = Some(3);

        assert_eq!(input.slice_index(3), output);
    }

    #[test]
    fn case_span_index_does_not_exist() {
        let input  = Span::new("foobar");
        let output = None;

        assert_eq!(input.slice_index(7), output);
    }

    #[test]
    fn case_span_empty_compare() {
        let input  = Span::new("");
        let output = CompareResult::Incomplete;

        assert_eq!(input.compare("foobar"), output);
    }

    #[test]
    fn case_span_find_substring_of_length_0() {
        let input  = Span::new("foobarbaz");
        let output = None;

        assert_eq!(input.find_substring(""), output);
    }

    #[test]
    fn case_span_find_substring_of_length_1() {
        let input  = Span::new("foobarbaz");
        let output = Some(3);

        assert_eq!(input.find_substring("b"), output);
    }

    #[test]
    fn case_span_find_substring_of_length_2() {
        let input  = Span::new("foobarbaz");
        let output = Some(3);

        assert_eq!(input.find_substring("ba"), output);
    }

    #[test]
    fn case_span_find_substring_of_length_3() {
        let input  = Span::new("foobarbaz");
        let output = Some(3);

        assert_eq!(input.find_substring("bar"), output);
    }

    #[test]
    fn case_span_find_substring_of_length_4() {
        let input  = Span::new("foobarbaz");
        let output = Some(3);

        assert_eq!(input.find_substring("barb"), output);
    }

    #[test]
    fn case_span_find_substring_out_of_bound() {
        let input  = Span::new("abc");
        let output = None;

        assert_eq!(input.find_substring("cd"), output);
    }

    #[test]
    fn case_span_compare_incomplete() {
        let input  = Span::new("foo");
        let output = CompareResult::Incomplete;

        assert_eq!(input.compare("foobar"), output);
    }

    #[test]
    fn case_span_compare_ok() {
        let input  = Span::new("foobar");
        let output = CompareResult::Ok;

        assert_eq!(input.compare("foobar"), output);
    }

    #[test]
    fn case_span_empty_compare_no_case() {
        let input  = Span::new("");
        let output = CompareResult::Incomplete;

        assert_eq!(input.compare("foobar"), output);
    }

    #[test]
    fn case_span_compare_no_case_incomplete() {
        let input  = Span::new("foo");
        let output = CompareResult::Incomplete;

        assert_eq!(input.compare("foobar"), output);
    }

    #[test]
    fn case_span_compare_no_case_ok() {
        let input  = Span::new("foobar");
        let output = CompareResult::Ok;

        assert_eq!(input.compare("foobar"), output);
    }

    #[test]
    fn case_span_slice_with_range() {
        let range  = 2..5;
        let input  = "foobar";
        let output = Span {
            offset: 2,
            line  : 1,
            column: 3,
            fragment : &input[range.clone()]
        };

        assert_eq!(Span::new(input).slice(range.clone()), output);
    }

    #[test]
    fn case_span_slice_with_range_to() {
        let range  = 2..;
        let input  = "foobar";
        let output = Span {
            offset: 2,
            line  : 1,
            column: 3,
            fragment : &input[range.clone()]
        };

        assert_eq!(Span::new(input).slice(range.clone()), output);
    }

    #[test]
    fn case_span_slice_with_range_from() {
        let range  = ..3;
        let input  = "foobar";
        let output = Span {
            offset: 0,
            line  : 1,
            column: 1,
            fragment : &input[range.clone()]
        };

        assert_eq!(Span::new(input).slice(range.clone()), output);
    }

    #[test]
    fn case_span_slice_with_range_full() {
        let range  = ..;
        let input  = "foobar";
        let output = Span {
            offset: 0,
            line  : 1,
            column: 1,
            fragment : &input[range.clone()]
        };

        assert_eq!(Span::new(input).slice(range.clone()), output);
    }

    #[test]
    fn case_span_in_parser() {
        named!(
            test<Span, Vec<Span>>,
            do_parse!(
                foo: ws!(tag!("foo")) >>
                bar: ws!(tag!("bar")) >>
                baz: many0!(ws!(tag!("baz"))) >>
                qux: tag!("qux") >>
                ({
                    let mut out = vec![foo, bar];
                    out.extend(baz);
                    out.push(qux);

                    out
                })
            )
        );

        let input = "foo bar\nbaz\n \n  baz   qux";
        let output = Ok((
            Span {
                offset: 25,
                line  : 4,
                column: 12,
                fragment : ""
            },
            vec![
                Span {
                    offset: 0,
                    line  : 1,
                    column: 1,
                    fragment : "foo",
                },
                Span {
                    offset: 4,
                    line  : 1,
                    column: 5,
                    fragment : "bar",
                },
                Span {
                    offset: 8,
                    line  : 2,
                    column: 1,
                    fragment : "baz",
                },
                Span {
                    offset: 16,
                    line  : 4,
                    column: 3,
                    fragment : "baz",
                },
                Span {
                    offset: 22,
                    line  : 4,
                    column: 9,
                    fragment : "qux",
                }
            ]
        ));

        assert_eq!(test(Span::new(input)), output);
    }
}