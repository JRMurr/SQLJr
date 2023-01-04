mod error;
mod table;
use std::collections::HashMap;

use error::{QueryExecutionError, SQLError};
use sql_jr_parser::ast::{parse_sql_query, SqlQuery};
use table::Table;

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

    pub fn run(&mut self, query: SqlQuery) -> Result<(), QueryExecutionError> {
        match query {
            SqlQuery::Select(select) => {
                let cols = select.fields;
                // for now skipping joins
                let table = select.tables.get(0).unwrap();
                let table = self
                    .tables
                    .get(table)
                    .ok_or(QueryExecutionError::TableNotFound(table.to_string()))?;

                println!("{:?}", self.tables);
                for (_id, row) in table.iter() {
                    let vals: Vec<&String> = cols.iter().map(|f| row.get(f).unwrap()).collect();

                    println!("{vals:?}")
                }
            }
            SqlQuery::Insert(insert) => {
                let Some(table) = self.tables.get_mut(&insert.table) else {
                    println!("no table");
                    return Ok(());
                };

                table.insert(insert.values);
            }
            SqlQuery::Create(create) => {
                let table = Table::new(create.columns);

                self.tables.insert(create.table, table);
            }
        }
        Ok(())
    }

    pub fn parse_and_run<'a>(&mut self, query: &'a str) -> Result<(), SQLError<'a>> {
        let query = parse_sql_query(query)?;

        let res = self.run(query)?;
        Ok(res)
    }
}
