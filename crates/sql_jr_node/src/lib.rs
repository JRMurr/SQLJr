#![deny(clippy::all)]
use sql_jr_execution::ExecResponse;

#[macro_use]
extern crate napi_derive;

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
