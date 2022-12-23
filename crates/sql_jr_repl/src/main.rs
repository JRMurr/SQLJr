use rustyline::{self, error::ReadlineError};

fn main() -> eyre::Result<()> {
    let mut rl = rustyline::Editor::<()>::new()?;

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => println!("Line: {line:?}"),
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
