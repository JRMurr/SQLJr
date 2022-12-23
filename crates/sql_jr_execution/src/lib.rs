use std::collections::HashMap;

use sql_jr_parser::ast::SqlQuery;

type Row = HashMap<usize, String>; // easy

#[derive(Debug, Default)]
pub struct Execution {
    data: Vec<Row>, // list
}

impl Execution {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn run(&mut self, query: SqlQuery) {
        match query {
            SqlQuery::Select(select) => {
                let cols: Vec<usize> = select
                    .fields
                    .into_iter()
                    .map(|f| f.parse().unwrap())
                    .collect();
                println!("{:?}", self.data);
                for row in self.data.iter() {
                    let vals: Vec<&String> = cols.iter().map(|f| row.get(f).unwrap()).collect();

                    println!("{vals:?}")
                }
            }
            SqlQuery::Insert(insert) => {
                let values = insert.values;
                if values.len() != 2 {
                    panic!("whats a schema. expect only 2 cols 0 and 1");
                }

                let row: Row = values.into_iter().enumerate().collect();
                println!("{:?}", row);

                self.data.push(row);
            }
        }
    }
}
