use nom::{
    self,
    branch::alt,
    character::complete::{char, multispace0},
    combinator::map,
    error::context,
    sequence::{preceded, tuple},
};
use serde::{Deserialize, Serialize};

use crate::{commands::*, parse::Parse};

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
