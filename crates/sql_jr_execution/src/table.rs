use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};
use sql_jr_parser::Column;

/// A Row stored in the db. Map of Column name to value
pub type Row = HashMap<String, String>;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct Table {
    /// row id to row
    rows: BTreeMap<usize, Row>,

    columns: Vec<Column>,
}

impl Table {
    pub fn new(columns: Vec<Column>) -> Self {
        Self {
            rows: BTreeMap::new(),
            columns,
        }
    }

    pub fn insert(&mut self, values: Vec<String>) {
        // TODO: make sql literal type that will be converted to the rust types

        let id = self
            .rows
            .last_key_value()
            .map_or(0, |(max_id, _)| max_id + 1);

        // TODO: make sure values is the right length and types
        let row: Row = values
            .into_iter()
            .zip(self.columns.iter())
            .map(|(v, col)| (col.name.to_owned(), v))
            .collect();

        self.rows.insert(id, row);
    }

    pub fn iter(&self) -> std::collections::btree_map::Iter<usize, Row> {
        self.rows.iter()
    }
}
