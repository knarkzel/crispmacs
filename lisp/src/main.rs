fn main() {
    dbg!(lisp::parse_and_eval("(define less (lambda (x y) (< x y))) (if (less 5 6) #t #f)"));
}
