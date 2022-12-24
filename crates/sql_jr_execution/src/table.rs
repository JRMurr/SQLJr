use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

type Row = HashMap<String, String>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum SqlTypeInfo {
    String, // maybe add size req?
    Int,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Column {
    name: String, // look into https://docs.rs/ustr/latest/ustr/
    type_info: SqlTypeInfo,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct Table {
    /// row id to row
    rows: BTreeMap<usize, Row>,

    columns: Vec<Column>,
}

impl Table {
    pub fn iter(&self) -> std::collections::btree_map::Iter<usize, Row> {
        self.rows.iter()
    }
}
