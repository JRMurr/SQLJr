use nom::{
    self,
    branch::alt,
    character::complete::{char, multispace0},
    combinator::map,
    error::context,
    sequence::{preceded, tuple},
};
use serde::{Deserialize, Serialize};

use crate::{
    commands::*,
    parse::{FormattedParseError, Parse},
};

/// All possible query types you can run
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum SqlQuery {
    Select(SelectStatement),
    Insert(InsertStatement),
    Create(CreateStatement),
}

impl<'a> Parse<'a> for SqlQuery {
    fn parse(input: crate::parse::RawSpan<'a>) -> crate::parse::ParseResult<'a, Self> {
        let (rest, (query, _, _, _)) = context(
            "Query",
            preceded(
                multispace0,
                tuple((
                    alt((
                        map(SelectStatement::parse, SqlQuery::Select),
                        map(InsertStatement::parse, SqlQuery::Insert),
                        map(CreateStatement::parse, SqlQuery::Create),
                    )),
                    multispace0,
                    char(';'),
                    multispace0,
                )),
            ),
        )(input)?;

        Ok((rest, query))
    }
}

// TODO: impl https://doc.rust-lang.org/std/str/trait.FromStr.html for SqlQuery
// https://github.com/Geal/nom/blob/main/doc/nom_recipes.md#implementing-fromstr
impl<'a> TryFrom<&'a str> for SqlQuery {
    // type Error = VerboseError<RawSpan<'a>>;
    type Error = FormattedParseError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match SqlQuery::parse_format_error(value) {
            Ok(query) => Ok(query),
            Err(e) => Err(e), // TODO: real error handling
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_error() {
        let query = SqlQuery::parse_from_raw("select fart;");
        assert!(query.is_err(), "expected parse to fail, got {query:?}");
    }

    #[test]
    fn test_select() {
        let expected = SelectStatement {
            tables: vec!["t1".to_string(), "t2".to_string()],
            fields: vec!["foo".to_string(), "bar".to_string()],
        };
        assert_eq!(
            SqlQuery::parse_from_raw("select foo, bar from t1,t2;")
                .unwrap()
                .1,
            SqlQuery::Select(expected)
        )
    }

    #[test]
    fn test_insert() {
        let expected = InsertStatement {
            table: "foo".to_string(),
            values: vec!["foo".to_string(), "bar".to_string()],
        };
        assert_eq!(
            SqlQuery::parse_from_raw("insert into foo values foo,bar;")
                .unwrap()
                .1,
            SqlQuery::Insert(expected)
        )
    }
}
