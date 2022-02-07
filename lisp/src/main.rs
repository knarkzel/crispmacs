fn main() {
    dbg!(lisp::parse_and_eval("(define triple (lambda (x) (+ x x x))) (triple 5)"));
}
