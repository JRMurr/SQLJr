use crate::parse::RawSpan;
use miette::Diagnostic;
use nom_supreme::error::{BaseErrorKind, ErrorTree, GenericErrorTree, StackContext};
use thiserror::Error;

pub(crate) type MyParseError<'a> = ErrorTree<RawSpan<'a>>;

#[derive(Error, Debug, Diagnostic)]
#[error("Parse Error")]
pub struct FormattedError<'b> {
    // need 'b since Diagnostic derive uses 'a
    #[source_code]
    src: &'b str,

    #[label("{kind}")]
    span: miette::SourceSpan,

    kind: BaseErrorKind<&'b str, Box<dyn std::error::Error + Send + Sync + 'static>>,

    #[related]
    others: Vec<FormattedErrorContext<'b>>,
}

#[derive(Error, Debug, Diagnostic)]
#[error("Parse Error Context")]
pub struct FormattedErrorContext<'b> {
    #[source_code]
    src: &'b str,

    #[label("{context}")]
    span: miette::SourceSpan,

    context: StackContext<&'b str>,
}

pub fn format_parse_error<'a>(input: &'a str, e: MyParseError<'a>) -> FormattedError<'a> {
    match e {
        GenericErrorTree::Base { location, kind } => {
            let offset = location.location_offset().into();
            FormattedError {
                src: input,
                span: miette::SourceSpan::new(offset, 0.into()),
                kind,
                others: Vec::new(),
            }
        }
        GenericErrorTree::Stack { base, contexts } => {
            let mut base = format_parse_error(input, *base);
            let mut contexts: Vec<FormattedErrorContext> = contexts
                .into_iter()
                .map(|(location, context)| {
                    let offset = location.location_offset().into();
                    FormattedErrorContext {
                        src: input,
                        span: miette::SourceSpan::new(offset, 0.into()),
                        context,
                    }
                })
                .collect();
            base.others.append(&mut contexts);
            base
        }
        GenericErrorTree::Alt(alt_errors) => {
            // get the error with the most context
            // TODO: figure out what to do on ties
            alt_errors
                .into_iter()
                .map(|e| format_parse_error(input, e))
                .max_by_key(|formatted| formatted.others.len())
                .unwrap()
        }
    }
}
