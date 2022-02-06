use lisp::parse_and_eval;

fn main() {
    dbg!(parse_and_eval("(or #f #f (= 1 2))"));
}
