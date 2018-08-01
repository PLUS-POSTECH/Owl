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

- Install `nightly-2018-07-07` version of Rust. Tarpc seems to require Rust nightly, and newer nightly seems to hate Diesel.
    - `rustup install nightly-2018-07-07`
- Install PostgreSQL library(libpq).
- Install [rustfmt](https://github.com/rust-lang-nursery/rustfmt#installation).
Copy files in [hooks](hooks) directory to `.git/hooks`.
    - `rustup component add rustfmt-preview --toolchain=nightly-2018-07-07`

## License

Licensed under either of
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be dual licensed as above, without any additional terms or conditions.
