# crispmacs

`crispmacs` is a WIP implementation of Emacs from scratch in Rust. It consists
of two parts: `crisp` and the `editor`. 

## Crisp

`crisp` is a Lisp that's based on Scheme that looks like Rust. It is currently
very bare-bones. To try out the language, clone the project and run the
repl:

### Examples

Recursive functions:

```lisp
>> (let sum (fn (x) (if (> x 0) (+ x (sum (- x 1))) x)))
>> (sum 10)
55
```

## Get started

```bash
git clone https://github.com/knarkzel/crispmacs
cd crispmacs/
```

### Running REPL

```bash
cd repl/
cargo run
```

### Running editor

Dependencies: [egui](https://github.com/emilk/egui#demo).

```bash
cd editor
cargo run
```

### Running editor in browser

``` bash
./scripts/setup_web.sh
./scripts/build_web.sh
./scripts/start_server.sh # open http://0.0.0.0:8080
```
