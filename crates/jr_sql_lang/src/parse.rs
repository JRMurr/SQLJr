use crate::ast::{SelectStatement, SqlQuery};
use nom::{
    self,
    branch::alt,
    bytes::complete::{tag_no_case, take_while1},
    character::complete::{char, multispace0, multispace1},
    combinator::map,
    error::{context, VerboseError},
    multi::separated_list1,
    sequence::tuple,
    IResult, Slice,
};
use nom_locate::LocatedSpan;
use serde::{Deserialize, Serialize};
// use serde::{Deserialize, Serialize};
type RawSpan<'a> = LocatedSpan<&'a str>;

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

type ParseResult<'a, T> = IResult<RawSpan<'a>, T, VerboseError<RawSpan<'a>>>;

// type ParseResult<'a, T> = IResult<&'a str, T>;

/// Parse a unquoted sql identifer
fn identifier(i: RawSpan) -> ParseResult<String> {
    context(
        "Identifier",
        map(take_while1(|c: char| c.is_alphanumeric()), |s: RawSpan| {
            s.fragment().to_string()
        }),
    )(i)
}

fn comma_sep_idents(i: RawSpan) -> ParseResult<Vec<String>> {
    separated_list1(tuple((multispace0, char(','), multispace0)), identifier)(i)
}

fn select_statment(i: RawSpan) -> ParseResult<SelectStatement> {
    let (remaining_input, (_, _, fields, _, _, _, tables)) = tuple((
        tag_no_case("select"),
        multispace1,
        context("fields", comma_sep_idents),
        multispace1,
        tag_no_case("from"),
        multispace1,
        context("tables", comma_sep_idents),
    ))(i)?;

    Ok((remaining_input, SelectStatement { fields, tables }))
}

pub fn sql_query(i: &str) -> ParseResult<SqlQuery> {
    let i = LocatedSpan::new(i);
    alt((map(select_statment, SqlQuery::Select),))(i)
}

impl<'a> TryFrom<&'a str> for SqlQuery {
    // type Error = VerboseError<RawSpan<'a>>;
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match sql_query(value) {
            Ok((_, query)) => Ok(query),
            Err(e) => Err(format!("{e:?}")), // TODO: real error handling
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error() {
        let query = std::convert::TryInto::<SqlQuery>::try_into("select fart");
        dbg!(query);
    }

    #[test]
    fn test_select() {
        let expected = SelectStatement {
            tables: vec!["t1".to_string(), "t2".to_string()],
            fields: vec!["foo".to_string(), "bar".to_string()],
        };
        assert_eq!(
            sql_query("select foo, bar from t1,t2").unwrap().1,
            SqlQuery::Select(expected)
        )
    }
}
