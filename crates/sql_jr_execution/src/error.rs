use miette::Diagnostic;
use sql_jr_parser::{error::FormattedError, value::Value, SqlTypeInfo};
use thiserror::Error;

// @Improvement: Could have the source code + spans for exec errors?

/// Errors during query execution
#[derive(Error, Debug, Diagnostic)]
#[error("Query Execution Error")]
pub enum QueryExecutionError {
    #[error("Table {0} was not found")]
    TableNotFound(String),

    #[error("Table {0} already exists")]
    TableAlreadyExists(String),

    #[error("Column {0} does not exist")]
    ColumnDoesNotExist(String),

    #[error("Value {1} can not be inserted into a {0} column")]
    InsertTypeMismatch(SqlTypeInfo, Value),
}

// @CLEANUP:  This feels weird to define here.
// Maybe make a crate for "full pipeline"

/// Errors at any point in the SQL "pipeline"
#[derive(Error, Debug, Diagnostic)]
#[error(transparent)]
pub enum SQLError<'a> {
    #[diagnostic(transparent)]
    QueryExecutionError(#[from] QueryExecutionError),

    #[diagnostic(transparent)]
    ParsingError(FormattedError<'a>),
}

// need a manual impl since the error #[from] seems sad on lifetimes
impl<'a> From<FormattedError<'a>> for SQLError<'a> {
    fn from(value: FormattedError<'a>) -> Self {
        SQLError::ParsingError(value)
    }
}
