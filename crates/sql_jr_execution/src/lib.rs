mod error;
mod table;
use std::collections::HashMap;

use derive_more::Display;
use error::{QueryExecutionError, SQLError};
use sql_jr_parser::ast::{parse_sql_query, SqlQuery};
use table::{Row, Table};
// TODO: Eventually might be good to have to do something like
// `query('..').fetch` to get values back the rest of the query types would
// return unit or some message
// see https://github.com/launchbadge/sqlx/tree/main#querying

#[derive(Debug, Display)]

pub enum ExecResponse {
    #[display(fmt = "{_0:?}")] // only show the values not "Select(...)"
    Select(Vec<Row>),
    Insert,
    Create,
}

#[derive(Debug, Default)]
pub struct Execution {
    tables: HashMap<String, Table>,
}

impl Execution {
    pub fn new() -> Self {
        Self {
            tables: HashMap::new(),
        }
    }

    pub fn run(&mut self, query: SqlQuery) -> Result<ExecResponse, QueryExecutionError> {
        match query {
            SqlQuery::Select(mut select) => {
                let _cols = select.fields;
                // for now skipping joins
                let table = select.tables.swap_remove(0);
                let table = self
                    .tables
                    .get(&table)
                    .ok_or(QueryExecutionError::TableNotFound(table))?;

                // let mut rows = Vec::with_capacity(table.len());
                // for (_id, row) in table.iter() {
                //     let vals: Vec<&String> = cols.iter().map(|f|
                // row.get(f).unwrap()).collect();

                //     println!("{vals:?}")
                // }
                let rows = table.iter().map(|(_id, row)| row.clone()).collect();
                Ok(ExecResponse::Select(rows))
            }
            SqlQuery::Insert(insert) => {
                let Some(table) = self.tables.get_mut(&insert.table) else {
                    return Err(QueryExecutionError::TableNotFound(insert.table))
                };

                table.insert(insert.values);
                Ok(ExecResponse::Insert)
            }
            SqlQuery::Create(create) => {
                let table = Table::new(create.columns);

                self.tables.insert(create.table, table);
                Ok(ExecResponse::Create)
            }
        }
    }

    pub fn parse_and_run<'a>(&mut self, query: &'a str) -> Result<ExecResponse, SQLError<'a>> {
        let query = parse_sql_query(query)?;

        let res = self.run(query)?;
        Ok(res)
    }
}
