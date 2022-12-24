mod table;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use sql_jr_parser::ast::SqlQuery;

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

    pub fn run(&mut self, query: SqlQuery) {
        match query {
            SqlQuery::Select(select) => {
                let cols = select.fields;

                let table = self.tables.get(select.tables.get(0).unwrap()).unwrap();

                println!("{:?}", self.tables);
                for (_id, row) in table.iter() {
                    let vals: Vec<&String> = cols.iter().map(|f| row.get(f).unwrap()).collect();

                    println!("{vals:?}")
                }
            }
            SqlQuery::Insert(insert) => {
                let Some(table) = self.tables.get_mut(&insert.table) else {
                    println!("no table");
                    return;
                };

                table.insert(insert.values);
            }
            SqlQuery::Create(create) => {
                let table = Table::new(create.columns);

                self.tables.insert(create.table, table);
            }
        }
    }
}
