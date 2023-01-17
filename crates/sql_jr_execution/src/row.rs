use std::{collections::HashMap, rc::Rc};

use derive_more::Display;

use crate::{error::QueryExecutionError, table::ColumnInfo};
/// A Row in a Query Response
#[derive(Debug, Clone, Display)]
#[display(fmt = "{:#?}", data)]
pub struct Row<'a> {
    id: usize,
    columns: Rc<ColumnInfo>,
    data: &'a HashMap<String, String>,
}

impl<'a> Row<'a> {
    pub fn new(columns: Rc<ColumnInfo>, id: usize, data: &'a HashMap<String, String>) -> Self {
        Self { id, columns, data }
    }

    /// Get a single value from the row
    ///
    /// # Panics
    ///
    /// Panics if the column does not exist
    /// See [`try_get`](Self::try_get) for a non-panicking
    /// version.
    pub fn get(&self, column: &String) -> String {
        self.try_get(column).unwrap()
    }

    /// Get a single value from the row
    pub fn try_get(&self, column: &String) -> Result<String, QueryExecutionError> {
        self.data.get(column).map_or_else(
            || Err(QueryExecutionError::ColumnDoesNotExist(column.to_owned())),
            |val| Ok(val.to_string()),
        )
    }
}
