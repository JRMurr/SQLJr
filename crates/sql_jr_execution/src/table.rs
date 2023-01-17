use std::{
    collections::{BTreeMap, HashMap},
    rc::Rc,
};

use serde::{Deserialize, Serialize};
use sql_jr_parser::Column;

use crate::row::Row;

/// A row stored in a table
type StoredRow = HashMap<String, String>;

/// List of column info
pub type ColumnInfo = Vec<Column>;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct Table {
    /// row id to row
    rows: BTreeMap<usize, StoredRow>,

    /// Column info for all columns in the table
    columns: ColumnInfo,
}

impl Table {
    // Create a table with the given column definitions
    pub fn new(columns: Vec<Column>) -> Self {
        Self {
            rows: BTreeMap::new(),
            columns,
        }
    }

    /// Insert values (a row) into the table
    ///
    /// Assumes the values are in the same order of the [`Column`]s passed to
    /// create
    pub fn insert(&mut self, values: Vec<String>) {
        let id = self
            .rows
            .last_key_value()
            .map_or(0, |(max_id, _)| max_id + 1);

        let row: StoredRow = values
            .into_iter()
            .zip(self.columns.iter())
            .map(|(v, col)| (col.name.to_owned(), v))
            .collect();

        self.rows.insert(id, row);
    }

    pub fn iter(&self) -> impl Iterator<Item = Row> {
        self.into_iter()
    }
}

impl<'a> IntoIterator for &'a Table {
    type Item = Row<'a>;

    type IntoIter = TableIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let col_info = Rc::new(self.columns.clone());

        TableIter::new(self.rows.iter(), col_info)
    }
}

/// Iterator of [`Row`]s from a table
pub(crate) struct TableIter<'a> {
    /// Underlying iterator over the btree_map
    map_iter: std::collections::btree_map::Iter<'a, usize, StoredRow>,
    /// The columns of the [`Table`]
    columns: Rc<ColumnInfo>,
}

impl<'a> TableIter<'a> {
    pub fn new(
        map_iter: std::collections::btree_map::Iter<'a, usize, StoredRow>,
        columns: Rc<ColumnInfo>,
    ) -> Self {
        Self { map_iter, columns }
    }
}

impl<'a> Iterator for TableIter<'a> {
    type Item = Row<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.map_iter
            .next()
            .map(|(id, data)| Row::new(self.columns.clone(), id.clone(), data))
    }
}
