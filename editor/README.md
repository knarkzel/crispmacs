# crispmacs

`crispmacs` is a WIP implementation of Emacs from scratch in Rust.

## Dependencies

See [egui](https://github.com/emilk/egui#demo).

## Running native

```bash
cargo run --release
```

### Running WASM

``` bash
./scripts/setup_web.sh
./scripts/build_web.sh
./scripts/start_server.sh # open http://0.0.0.0:8080
```

The finished web app is found in the `docs/`.
