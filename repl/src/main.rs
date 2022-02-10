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
                // dbg!(crisp::parse(&line));
                match crisp::parse_and_eval(&line, &mut context) {
                    Ok(Ok(expr)) => {
                        if expr.len() > 0 {
                            for expr in expr {
                                println!("{}", expr)
                            }
                        }
                    }
                    Ok(Err(e)) => println!("Evaluation error: {}", e),
                    Err(e) => println!("Parsing error: {}", e),
                }
            }
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Readline error: {:?}", err);
                break;
            }
        }
    }
}
