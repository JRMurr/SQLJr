use std::collections::HashMap;

use napi_derive::napi;
use sql_jr_execution::{ExecResponse, Execution};

#[napi(js_name = "Execution")]
pub struct NodeExec {
    execution: Execution,
}

// to appease clippy
impl Default for NodeExec {
    fn default() -> Self {
        Self::new()
    }
}

/// A List of rows returned by the query.
/// Each row is a map of col => data as string
type QueryRes = Vec<HashMap<String, String>>;

#[napi]
impl NodeExec {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            execution: Execution::new(),
        }
    }

    #[napi]
    pub fn query(&mut self, query: String) -> napi::Result<QueryRes> {
        use napi::{Error, Status};
        let res = self
            .execution
            .parse_and_run(&query)
            // Probably a good idea to impl From<SqlError<_> for napi::Error in sql_jr_execution
            // gated behind a napi feature flag
            .map_err(|e| Error::new(Status::GenericFailure, format!("{}", e)))?;

        Ok(match res {
            ExecResponse::Select(table_iter) => {
                let columns: Vec<String> = table_iter
                    .columns
                    .iter()
                    .map(|col| col.name.to_string()) // Involves a clone, maybe we can make col name a Cow?
                    .collect();

                table_iter
                    .map(|row| {
                        columns
                            .iter()
                            .map(move |col| (col.clone(), row.get(col).to_string()))
                            .collect()
                    })
                    .collect()
            }
            _ => Vec::new(),
        })
    }
}

#[napi]
pub fn test() -> Vec<Vec<String>> {
    let mut exec = sql_jr_execution::Execution::new();
    exec.parse_and_run(
        "
      CREATE TABLE foo (
        col1 int,
        col2 string
      );
    ",
    )
    .expect("create works..");

    exec.parse_and_run(
        "
        INSERT INTO foo
        VALUES
            1, 'aString';
      ",
    )
    .expect("insert 1 works..");

    exec.parse_and_run(
        "
        INSERT INTO foo
        VALUES
            4, 'aDiffString with spaces';
    ",
    )
    .expect("insert 2 works..");

    let res = exec
        .parse_and_run(
            "
        SELECT
          col1,
          col2
        FROM
            foo;
        ",
        )
        .expect("select works");

    match res {
        ExecResponse::Select(table_iter) => {
            let columns: Vec<String> = table_iter
                .columns
                .iter()
                .map(|col| col.name.to_string()) // Involves a clone, maybe we can make col name a Cow?
                .collect();

            let rows: Vec<Vec<_>> = table_iter
                .map(|row| {
                    columns
                        .iter()
                        .map(move |col| row.get(col).to_string())
                        .collect()
                })
                .collect();

            rows
        }
        _ => unreachable!(),
    }
}
