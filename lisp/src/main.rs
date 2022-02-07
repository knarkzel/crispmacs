fn main() {
    dbg!(lisp::parse_and_eval("((lambda (x y) (< x y)) 5 6)"));
}
