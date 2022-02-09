use rustyline::Editor;
use rustyline::error::ReadlineError;

fn main() {
    let mut rl = Editor::<()>::new();
    let mut context = crisp::Context::default();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                match crisp::parse_and_eval(&line, &mut context) {
                    Ok(expr) => {
                        if expr.len() > 0 {
                            for expr in expr {
                                println!("{}", expr)
                            }
                        }
                    }
                    Err(e) => println!("Error occurred: {}", e),
                }
            }
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
