fn main() {
    dbg!(lisp::parse_and_eval("(define sum1 123) (+ sum1 sum1 sum1)"));
}
