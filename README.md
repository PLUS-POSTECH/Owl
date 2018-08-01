# Owl

Management tool for Attack and Defense CTFs

## Testing, Building, and Running

Owl is written with Rust, and utilizes Cargo as a building and testing system.

You can test, build, run using the following command:

```
cargo test
cargo build
cargo run
```

## Development

- Install nightly version of Rust. Tarpc seems to require Rust nightly.
- Install PostgreSQL library(libpq).
- Install [clippy](https://github.com/rust-lang-nursery/rust-clippy#as-a-cargo-subcommand-cargo-clippy) and [rustfmt](https://github.com/rust-lang-nursery/rustfmt#installation).
Copy files in [hooks](hooks) directory to `.git/hooks`.
    - `rustup component add clippy-preview --toolchain=nightly`
    - `rustup component add rustfmt-preview --toolchain=nightly`

## License

Licensed under either of
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be dual licensed as above, without any additional terms or conditions.
