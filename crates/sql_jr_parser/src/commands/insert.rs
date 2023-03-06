use nom::{
    character::complete::multispace1,
    error::context,
    sequence::{preceded, tuple},
};
use nom_supreme::{tag::complete::tag_no_case, ParserExt};
use serde::{Deserialize, Serialize};

use crate::{
    parse::{comma_sep, identifier, Parse, ParseResult, RawSpan},
    value::Value,
};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct InsertStatement {
    pub table: String,
    pub values: Vec<Value>,
} // TODO: impl display

impl<'a> Parse<'a> for InsertStatement {
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self> {
        let (remaining_input, (_, _, table, _, values)) = context(
            "Insert Statement",
            tuple((
                tag_no_case("insert"),
                preceded(multispace1, tag_no_case("into")),
                preceded(multispace1, identifier.context("Table Name")),
                preceded(multispace1, tag_no_case("values")),
                preceded(multispace1, comma_sep(Value::parse).context("Values")),
            )),
        )(input)?;

        Ok((remaining_input, InsertStatement { table, values }))
    }
}
