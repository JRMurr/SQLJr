mod table;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use sql_jr_parser::ast::SqlQuery;

use table::Table;

#[derive(Debug, Default)]
pub struct Execution {
    data: HashMap<String, Table>,
}

impl Execution {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn run(&mut self, query: SqlQuery) {
        match query {
            SqlQuery::Select(select) => {
                let cols = select.fields;

                let table = self.data.get(select.tables.get(0).unwrap()).unwrap();

                println!("{:?}", self.data);
                for (id, row) in table.iter() {
                    let vals: Vec<&String> = cols.iter().map(|f| row.get(f).unwrap()).collect();

                    println!("{vals:?}")
                }
            }
            SqlQuery::Insert(insert) => {
                // let values = insert.values;
                // if values.len() != 2 {
                //     panic!("whats a schema. expect only 2 cols 0 and 1");
                // }

                // let row: Row = values.into_iter().enumerate().collect();
                // self.data.push(row);

                todo!()
            }
            SqlQuery::Create(create) => {
                todo!()
            }
        }
    }
}
