use display::display_response;
use miette::{Context, GraphicalReportHandler, IntoDiagnostic};
use rustyline::error::ReadlineError;

mod display;
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
                let res = exec.parse_and_run(line);
                match res {
                    Ok(exec_res) => display_response(exec_res),
                    Err(e) => {
                        let mut s = String::new();
                        GraphicalReportHandler::new()
                            .with_cause_chain()
                            .with_context_lines(10)
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
