use nom::{
    self,
    bytes::complete::take_while1,
    character::complete::{char, multispace0},
    combinator::{all_consuming, map, peek},
    multi::separated_list1,
    sequence::{pair, tuple},
    Finish, IResult, Slice,
};
use nom_locate::LocatedSpan;
use nom_supreme::tag::complete::tag_no_case;
use serde::{Deserialize, Serialize};

use crate::error::{format_parse_error, FormattedError, MyParseError};
// use serde::{Deserialize, Serialize};
pub type RawSpan<'a> = LocatedSpan<&'a str>;

pub type FormattedParseError = String; // TODO: real error

// stealing more code from gdlk...
// https://github.com/LucasPickering/gdlk/blob/1fb8c9b988fd86be8541a66b8e079a1b9d133cf4/crates/core/src/util.rs#L18
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    /// Distance into the source at which this span starts. Starts at `0`.
    pub offset: usize,
    /// Number of characters that this span includes.
    pub length: usize,
    /// The line number that this span starts on, starting at `1`.
    pub start_line: usize,
    /// The column that this span starts at, starting at `1`.
    pub start_col: usize,
    /// The line number that this span ends on, starting at `1`.
    pub end_line: usize,
    /// The column that this span ends at, starting at `1`.
    pub end_col: usize,
}

impl<'a> From<RawSpan<'a>> for Span {
    fn from(value: RawSpan<'a>) -> Self {
        let len = value.fragment().len();
        let end = value.slice(len..);

        Self {
            offset: value.location_offset(),
            length: value.fragment().len(),
            start_line: value.location_line() as usize,
            start_col: value.get_column(),
            end_line: end.location_line() as usize,
            end_col: end.get_column(),
        }
    }
}

pub type ParseResult<'a, T> = IResult<RawSpan<'a>, T, MyParseError<'a>>;

/// Implement the parse function to more easily convert a span into a sql
/// command
pub(crate) trait Parse<'a>: Sized {
    /// Parse the given span into self
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self>;

    // Helper method to convert a raw str into a raw span and parse
    fn parse_from_raw(input: &'a str) -> ParseResult<'a, Self> {
        let i = LocatedSpan::new(input);
        Self::parse(i)
    }

    fn parse_format_error(i: &'a str) -> Result<Self, FormattedError<'a>> {
        let input = LocatedSpan::new(i);
        // https://github.com/fflorent/nom_locate/issues/36#issuecomment-1013469728
        match all_consuming(Self::parse)(input).finish() {
            Ok((_, query)) => Ok(query),
            Err(e) => Err(format_parse_error(i, e)),
        }
    }
}

/// Parse a unquoted sql identifer
pub(crate) fn identifier(i: RawSpan) -> ParseResult<String> {
    map(take_while1(|c: char| c.is_alphanumeric()), |s: RawSpan| {
        s.fragment().to_string()
    })(i)
}

/// Check if the input has the passed in tag
/// if so run the parser supplied (with the peeked tag still expected)
/// and cut on error
///
/// This is useful for alts so we stop on errors
pub(crate) fn peek_then_cut<'a, T, O, E, F>(
    peek_tag: T,
    f: F,
) -> impl FnMut(RawSpan<'a>) -> IResult<RawSpan<'a>, O, E>
where
    T: nom::InputLength + Clone,
    F: nom::Parser<RawSpan<'a>, O, E>,
    E: nom::error::ParseError<RawSpan<'a>> + nom_supreme::tag::TagError<RawSpan<'a>, T>,
    LocatedSpan<&'a str>: nom::Compare<T>,
{
    map(pair(peek(tag_no_case(peek_tag)), f), |(_, f_res)| f_res)
}

pub(crate) fn comma_sep<'a, O, E, F>(
    f: F,
) -> impl FnMut(RawSpan<'a>) -> IResult<RawSpan<'a>, Vec<O>, E>
where
    F: nom::Parser<RawSpan<'a>, O, E>,
    E: nom::error::ParseError<RawSpan<'a>>,
{
    separated_list1(tuple((multispace0, char(','), multispace0)), f)
}
