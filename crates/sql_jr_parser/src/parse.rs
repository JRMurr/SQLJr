use nom::{
    self,
    bytes::complete::take_while1,
    character::complete::{char, multispace0},
    combinator::map,
    error::{context, convert_error, VerboseError},
    multi::separated_list1,
    sequence::tuple,
    Finish, IResult, Slice,
};
use nom_locate::LocatedSpan;
use serde::{Deserialize, Serialize};
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

pub type ParseResult<'a, T> = IResult<RawSpan<'a>, T, VerboseError<RawSpan<'a>>>;

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

    fn parse_format_error(i: &'a str) -> Result<Self, String> {
        let input = LocatedSpan::new(i);
        // https://github.com/fflorent/nom_locate/issues/36#issuecomment-1013469728
        match Self::parse(input).finish() {
            Ok((_, query)) => Ok(query),
            Err(e) => {
                let errors = e
                    .errors
                    .into_iter()
                    .map(|(input, error)| (*input.fragment(), error))
                    .collect();

                Err(convert_error(i, VerboseError { errors }))
            } // TODO: real error handling
        }
    }
}

/// Parse a unquoted sql identifer
pub(crate) fn identifier(i: RawSpan) -> ParseResult<String> {
    context(
        "Identifier",
        map(take_while1(|c: char| c.is_alphanumeric()), |s: RawSpan| {
            s.fragment().to_string()
        }),
    )(i)
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
