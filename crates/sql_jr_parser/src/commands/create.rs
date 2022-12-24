use nom::{
    branch::alt,
    bytes::complete::tag_no_case,
    character::complete::{char, multispace1},
    combinator::map,
    error::context,
    sequence::{pair, preceded, separated_pair, tuple},
};
use serde::{Deserialize, Serialize};

use crate::parse::{comma_sep, identifier, Parse, ParseResult, RawSpan};

// TODO: feels like SqlTypeInfo + column might wanna live elsewhere. Its parse
// thing but also needed for exec/storage? Maybe make a types crate?

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum SqlTypeInfo {
    String, // maybe add size req?
    Int,
}

impl<'a> Parse<'a> for SqlTypeInfo {
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self> {
        context(
            "Column Type",
            alt((
                map(tag_no_case("string"), |_| Self::String),
                map(tag_no_case("int"), |_| Self::Int),
            )),
        )(input)
    }
}

// TODO: should this be called create column since thats whats the parse is tied
// to?
#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Column {
    pub name: String, // look into https://docs.rs/ustr/latest/ustr/
    pub type_info: SqlTypeInfo,
}

impl<'a> Parse<'a> for Column {
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self> {
        context(
            "Create Column",
            map(
                separated_pair(
                    context("Column Name", identifier),
                    multispace1,
                    SqlTypeInfo::parse,
                ),
                |(name, type_info)| Self { name, type_info },
            ),
        )(input)
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct CreateStatement {
    pub table: String,
    pub columns: Vec<Column>,
}

fn column_definitions(input: RawSpan<'_>) -> ParseResult<'_, Vec<Column>> {
    context(
        "Column Definitions",
        map(
            tuple((char('('), comma_sep(Column::parse), char(')'))),
            |(_, cols, _)| cols,
        ),
    )(input)
}

impl<'a> Parse<'a> for CreateStatement {
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self> {
        map(
            separated_pair(
                preceded(
                    tuple((
                        tag_no_case("create"),
                        multispace1,
                        tag_no_case("table"),
                        multispace1,
                    )),
                    context("table", identifier),
                ),
                multispace1,
                column_definitions,
            ),
            |(table, columns)| Self { table, columns },
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let expected = CreateStatement {
            table: "foo".into(),
            columns: vec![
                Column {
                    name: "col1".into(),
                    type_info: SqlTypeInfo::Int,
                },
                Column {
                    name: "col2".into(),
                    type_info: SqlTypeInfo::String,
                },
                Column {
                    name: "col3".into(),
                    type_info: SqlTypeInfo::String,
                },
            ],
        };
        assert_eq!(
            CreateStatement::parse_from_raw(
                "CREATE TABLE foo (col1 int, col2 string, col3 string)"
            )
            .unwrap()
            .1,
            expected
        )
    }
}
