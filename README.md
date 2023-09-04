# Lang-C

[![Documentation](https://docs.rs/lang-c/badge.svg)](https://docs.rs/lang-c)

Ligtweight parser of C language for Rust users. Almost full support for C11 revision of the language.
Several GCC and Clang extensions are also supported as an option.

```rust
extern crate lang_c;
use lang_c::driver::{Config, parse}; 

fn main() {
    let config = Config::default();
    println!("{:?}", parse(&config, "example.c"));
}
```

# Bugs

Just open an issue, bug reports and patches are most welcome. 

## License

Dual-licenced under Apache 2.0 or MIT licenses (see `LICENSE-APACHE` and `LICENSE-MIT` for legal terms).

## Development

A number of external tools are used during development:

- GNU make
- rustfmt
- [rust-peg](https://github.com/kevinmehall/rust-peg) 0.5.4

Parser (`src/parser.rs`) is built from a PEG grammar in `grammar.rustpeg`. It is updated manually and then 
committed, not generated on every build, thus no `rust-peg` in the list of dependencies.

For debugging purposes, it is handy to have a version rust-peg built with tracing enabled.

A makefile is used to script the development process:

- `make` update parser, build the library and run the tests;
- `make trace` rebuilds parser using `rust-peg-trace`, which is expected to be a version of `rust-peg` command with `trace` feature enabled
- `make check` can be used as pre-commit git hook to make sure parser is up to date
