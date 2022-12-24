use core::fmt;

use nom::{
    bytes::complete::tag_no_case,
    character::complete::multispace1,
    error::context,
    sequence::{preceded, tuple},
};
use serde::{Deserialize, Serialize};

use crate::parse::{comma_sep, identifier, Parse, ParseResult, RawSpan};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct InsertStatement {
    pub table: String,
    pub values: Vec<String>, // for now just Strings
} // TODO: impl display

impl<'a> Parse<'a> for InsertStatement {
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self> {
        let (remaining_input, (_, _, table, _, values)) = context(
            "Insert Statement",
            tuple((
                tag_no_case("insert"),
                preceded(multispace1, tag_no_case("into")),
                preceded(multispace1, context("table", identifier)),
                preceded(multispace1, tag_no_case("values")),
                preceded(multispace1, context("values", comma_sep(identifier))),
            )),
        )(input)?;

        Ok((remaining_input, InsertStatement { table, values }))
    }
}
