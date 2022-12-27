use nom::{
    self,
    bytes::complete::take_while1,
    character::complete::{char, multispace0},
    combinator::{all_consuming, map, peek},
    multi::separated_list1,
    sequence::{pair, tuple},
    Finish, IResult,
};
use nom_locate::LocatedSpan;
use nom_supreme::tag::complete::tag_no_case;

use crate::error::{format_parse_error, FormattedError, MyParseError};

pub type RawSpan<'a> = LocatedSpan<&'a str>;

pub type ParseResult<'a, T> = IResult<RawSpan<'a>, T, MyParseError<'a>>;

/// Implement the parse function to more easily convert a span into a sql
/// command
pub trait Parse<'a>: Sized {
    /// Parse the given span into self
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self>;

    // Helper method to convert a raw str into a raw span and parse
    fn parse_from_raw(input: &'a str) -> ParseResult<'a, Self> {
        let i = LocatedSpan::new(input);
        Self::parse(i)
    }

    fn parse_format_error(i: &'a str) -> Result<Self, FormattedError<'a>> {
        let input = LocatedSpan::new(i);
        match all_consuming(Self::parse)(input).finish() {
            Ok((_, query)) => Ok(query),
            Err(e) => Err(format_parse_error(i, e)),
        }
    }
}

/// Parse a unquoted sql identifier
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
