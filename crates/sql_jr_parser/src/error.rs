use nom::error::VerboseError;

use crate::parse::RawSpan;

type MyParseError<'a> = VerboseError<RawSpan<'a>>;

pub fn format_parse_error<'a>(e: MyParseError<'a>) {}
