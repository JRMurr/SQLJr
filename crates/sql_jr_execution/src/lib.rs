use std::collections::HashMap;

use sql_jr_parser::ast::SqlQuery;

type Row = HashMap<String, String>; // easy

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
            SqlQuery::Select(select) => println!("select: {select}"),
            SqlQuery::Insert(insert) => println!("insert: {insert:?}"),
        }
    }
}
