use rustyline::{self, error::ReadlineError};
use sql_jr_parser::{self, ast::SqlQuery, parse::ParseError};
fn main() -> eyre::Result<()> {
    let mut rl = rustyline::Editor::<()>::new()?;

    let mut exec = sql_jr_execution::Execution::new();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let line: &str = line.as_ref();
                let query: Result<SqlQuery, ParseError> = line.try_into();
                if let Ok(q) = query {
                    exec.run(q);
                }
            }
            Err(ReadlineError::Interrupted) => {
                // CTRL-C so just skip
            }
            Err(ReadlineError::Eof) => {
                //"CTRL-D"
                break;
            }
            Err(err) => {
                println!("Error: {err:?}");
                break;
            }
        }
    }

    Ok(())
}
