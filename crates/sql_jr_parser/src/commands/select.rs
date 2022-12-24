use core::fmt;

use nom::{
    bytes::complete::tag_no_case, character::complete::multispace1, error::context, sequence::tuple,
};
use serde::{Deserialize, Serialize};

use crate::parse::{comma_sep, identifier, Parse, ParseResult, RawSpan};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct SelectStatement {
    pub tables: Vec<String>,
    pub fields: Vec<String>,
}

impl fmt::Display for SelectStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SELECT ")?;

        write!(f, "{}", self.fields.join(", "))?;

        write!(f, " FROM ")?;

        write!(f, "{}", self.tables.join(", "))?;

        Ok(())
    }
}

impl<'a> Parse<'a> for SelectStatement {
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self> {
        let (remaining_input, (_, _, fields, _, _, _, tables)) = context(
            "Select Statement",
            tuple((
                tag_no_case("select"),
                multispace1,
                context("fields", comma_sep(identifier)),
                multispace1,
                tag_no_case("from"),
                multispace1,
                context("tables", comma_sep(identifier)),
            )),
        )(input)?;

        Ok((remaining_input, SelectStatement { fields, tables }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select() {
        let expected = SelectStatement {
            tables: vec!["t1".to_string(), "t2".to_string()],
            fields: vec!["foo".to_string(), "bar".to_string()],
        };

        assert_eq!(
            SelectStatement::parse_from_raw("select foo, bar from t1,t2;")
                .unwrap()
                .1,
            expected
        )
    }
}
