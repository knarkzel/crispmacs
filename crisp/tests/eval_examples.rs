use crisp::*;
use std::fs::read_dir;

#[test]
#[throws]
fn eval_files() {
    for path in read_dir("tests/examples")? {
        let mut context = Context::default();
        let input = std::fs::read_to_string(path?.path())?;
        match parse_and_eval(&input, &mut context) {
            Ok(_) => assert!(true),
            Err(e) => { println!("{}", e); assert!(false) }
        }
    }
}
