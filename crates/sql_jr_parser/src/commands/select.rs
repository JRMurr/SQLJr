use core::fmt;

use nom::{character::complete::multispace1, error::context, sequence::tuple};
use nom_supreme::{tag::complete::tag_no_case, ParserExt};
use serde::{Deserialize, Serialize};

use crate::parse::{comma_sep, identifier, Parse, ParseResult, RawSpan};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct SelectStatement {
    pub table: String,
    pub fields: Vec<String>,
}

impl fmt::Display for SelectStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SELECT ")?;

        write!(f, "{}", self.fields.join(", "))?;

        write!(f, " FROM ")?;

        write!(f, "{}", self.table)?;

        Ok(())
    }
}

impl<'a> Parse<'a> for SelectStatement {
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self> {
        let (remaining_input, (_, _, fields, _, _, _, table)) = context(
            "Select Statement",
            tuple((
                tag_no_case("select"),
                multispace1,
                comma_sep(identifier).context("Select Columns"),
                multispace1,
                tag_no_case("from"),
                multispace1,
                identifier.context("From Table"),
            )),
        )(input)?;

        Ok((remaining_input, SelectStatement { fields, table }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select() {
        let expected = SelectStatement {
            table: "t1".to_string(),
            fields: vec!["foo".to_string(), "bar".to_string()],
        };

        assert_eq!(
            SelectStatement::parse_from_raw("select foo, bar from t1;")
                .unwrap()
                .1,
            expected
        )
    }
}
