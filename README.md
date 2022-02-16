# crispmacs

`crispmacs` is a WIP implementation of Emacs from scratch in Rust. It consists
of two parts: `crisp` and the `editor`. 

## Crisp

`crisp` is a programming language based on Lisp that takes more inspiration
from Rust (keywords for instance). For documentation, see [DOCS.md](./DOCS.md).

## Editor

`crispmacs` works on the desktop and in the browser. You can try it in
the browser here: [crispmacs](https://knarkzel.github.io/crispmacs/).

For native, install following dependencies:

```bash
apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libssl-dev
```

Then run following:

```bash
git clone https://github.com/knarkzel/crispmacs
cd crispmacs/editor
cargo run
```
