use miette::{Context, GraphicalReportHandler, IntoDiagnostic};
use rustyline::{self, error::ReadlineError};
use sql_jr_parser::{self, ast::parse_sql_query};

const HISTORY_FILE: &str = "./history.txt";

fn main() -> miette::Result<()> {
    let mut rl = rustyline::Editor::<()>::new()
        .into_diagnostic()
        .wrap_err("Initilizing REPL")?;
    if rl.load_history(HISTORY_FILE).is_err() {
        println!("No previous history.");
    }
    let mut exec = sql_jr_execution::Execution::new();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let line: &str = line.as_ref();
                let query = parse_sql_query(line);
                match query {
                    Ok(q) => exec.run(q),
                    Err(e) => {
                        let mut s = String::new();
                        GraphicalReportHandler::new()
                            .render_report(&mut s, &e)
                            .unwrap();
                        println!("{s}");
                    }
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
    rl.save_history(HISTORY_FILE)
        .into_diagnostic()
        .wrap_err("Saving REPL history")?;

    Ok(())
}
