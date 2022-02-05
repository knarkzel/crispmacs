# uemacs

`uemacs` is a project showing how to implement the bare-bones of Emacs.

## Crates

[nom](https://docs.rs/nom/7.1.0/nom/index.html) for parsing the programming
language. [ropey](https://docs.rs/ropey/1.3.2/ropey/index.html) or [xi-rope](https://docs.rs/xi-rope/0.3.0/xi_rope/index.html)
for efficient insertion and deletion, aka. editing (Emacs uses gap-buffers,
which has its positives and negatives.) [egui](https://docs.rs/egui/0.16.1/egui/)
for user-interface.

## Goals

- Hopefully less than 1000 SLOC
