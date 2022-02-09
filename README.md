# crispmacs

`crispmacs` is a WIP implementation of Emacs from scratch in Rust.

## Crisp

`crisp` is a Lisp that's based on Scheme that looks like Rust. It is currently
very bare-bones. To try out the language, clone the project and run the
repl:

```bash
git clone https://github.com/knarkzel/crispmacs
cd crispmacs/repl/
cargo run
```

### Examples

Recursive functions:

```lisp
>> (let sum (fn (x) (if (> x 0) (+ x (sum (- x 1))) x)))
>> (sum 10)
55
```

## Editor

- [ropey](https://docs.rs/ropey/1.3.2/ropey/index.html) as data structure
- [egui](https://docs.rs/egui/0.16.1/egui/) for user interface
