fn main() {
    dbg!(lisp::parse_and_eval("(define triple (lambda (x y) (+ x x x))) (triple 5)"));
}
