use display::display_response;
use miette::{Context, GraphicalReportHandler, IntoDiagnostic};
use rustyline::{
    completion::FilenameCompleter, error::ReadlineError, highlight::Highlighter, Completer,
    CompletionType, Config, Editor, Helper, Hinter, Validator,
};
use sql_jr_execution::{ExecResponse, Execution, SQLError};

mod display;
const HISTORY_FILE: &str = "./history.txt";

fn display_exec_res(res: Result<ExecResponse, SQLError>) {
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

fn handle_line(line: String, exec: &mut Execution) {
    let line: &str = line.as_ref();
    if !line.starts_with("/i") {
        display_exec_res(exec.parse_and_run(line));
    } else {
        let words: Vec<_> = line.split_ascii_whitespace().collect();
        match words.as_slice() {
            ["/i", file_path] => {
                let contents = std::fs::read_to_string(file_path).expect("sad reading file");
                display_exec_res(exec.parse_multiple_and_run(contents.as_ref()));
            }
            _ => todo!(),
        }
    };
}

#[derive(Completer, Helper, Hinter, Validator)]
struct MyHelper(#[rustyline(Completer)] FilenameCompleter); // TODO: other helper traits

impl Highlighter for MyHelper {}

fn main() -> miette::Result<()> {
    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .build();
    let mut rl = Editor::with_config(config)
        .into_diagnostic()
        .wrap_err("Initilizing REPL")?;

    rl.set_helper(Some(MyHelper(FilenameCompleter::new())));

    if rl.load_history(HISTORY_FILE).is_err() {
        println!("No previous history.");
    }
    let mut exec = sql_jr_execution::Execution::new();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).into_diagnostic()?;
                handle_line(line, &mut exec)
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
