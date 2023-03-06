use std::{
    collections::{BTreeMap, HashMap},
    rc::Rc,
};

use serde::{Deserialize, Serialize};
use sql_jr_parser::{value::Value, Column};

use crate::{error::QueryExecutionError, row::Row};

/// A row stored in a table
#[derive(Debug, Clone, Default, Serialize, Deserialize, derive_more::From)]
pub struct StoredRow {
    data: HashMap<String, Value>, // TODO: store bytes instead of the value enum
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, derive_more::From)]
pub struct ColumnInfo {
    columns: Vec<Column>,
}

impl ColumnInfo {
    pub fn iter(&self) -> impl Iterator<Item = &Column> {
        self.columns.iter()
    }

    pub fn find_column(&self, column_name: &String) -> Result<&Column, QueryExecutionError> {
        self.iter()
            .find(|col| col.name == *column_name)
            .ok_or_else(|| QueryExecutionError::ColumnDoesNotExist(column_name.to_owned()))
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct Table {
    /// row id to row
    rows: BTreeMap<usize, StoredRow>,

    /// Column info for all columns in the table
    columns: ColumnInfo,
}

/// Selecting columns from a table will return the columns selected and an
/// iterator over the rows
#[derive(Debug)]
pub struct SelectResponse<'a> {
    pub column_info: std::rc::Rc<ColumnInfo>,
    pub rows: TableIter<'a>,
}

impl Table {
    // Create a table with the given column definitions
    pub fn new(columns: Vec<Column>) -> Self {
        Self {
            rows: BTreeMap::new(),
            columns: columns.into(),
        }
    }

    /// Insert values (a row) into the table
    ///
    /// Assumes the values are in the same order of the [`Column`]s passed to
    /// create
    pub fn insert(&mut self, values: Vec<Value>) {
        let id = self
            .rows
            .last_key_value()
            .map_or(0, |(max_id, _)| max_id + 1);

        let row: HashMap<_, _> = values
            .into_iter()
            .zip(self.columns.iter())
            .map(|(v, col)| (col.name.to_owned(), v))
            .collect();

        self.rows.insert(id, row.into());
    }

    pub fn select(&self, columns: Vec<String>) -> Result<TableIter, QueryExecutionError> {
        let selected_columns = columns
            .into_iter()
            .map(|column_name| {
                self.columns
                    .find_column(&column_name)
                    .map(|col| col.clone())
            })
            .collect::<Result<Vec<_>, _>>()?;

        let col_info: Rc<ColumnInfo> = Rc::new(selected_columns.into());

        Ok(TableIter::new(self.rows.iter(), col_info))
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
#[derive(Debug)]
pub struct TableIter<'a> {
    /// Underlying iterator over the btree_map
    map_iter: std::collections::btree_map::Iter<'a, usize, StoredRow>,
    /// The columns of the [`Table`]
    pub columns: Rc<ColumnInfo>,
}

impl<'a> TableIter<'a> {
    pub(crate) fn new(
        map_iter: std::collections::btree_map::Iter<'a, usize, StoredRow>,
        columns: Rc<ColumnInfo>,
    ) -> Self {
        Self { map_iter, columns }
    }
}

impl<'a> Iterator for TableIter<'a> {
    type Item = Row<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.map_iter.next().map(|(id, data)| {
            let projected_data = data
                .data
                .iter()
                .filter_map(|(key, value)| self.columns.find_column(key).ok().map(|_| (key, value)))
                .collect();

            Row::new(self.columns.clone(), *id, projected_data)
        })
    }
}
