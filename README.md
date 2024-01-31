# iseven_api

[![Crates.io Version](https://img.shields.io/crates/v/iseven_api?style=flat-square)](https://crates.io/crates/iseven_api)
[![Crates.io License](https://img.shields.io/crates/l/iseven_api?style=flat-square)](#license)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/megascrapper/iseven-api-rust/build.yml?style=flat-square)
](https://github.com/megascrapper/iseven-api-rust/actions/workflows/build.yml)

A Rust wrapper for [isEven API](https://isevenapi.xyz/).

Includes the library as well as a simple command-line front end.

## Command line app

### Installation

```
cargo install iseven_api --all-features
```

## Building from source

```
git clone https://github.com/megascrapper/iseven-api-rust
cd iseven-api
cargo build --all-features
```

### Command line usage

```
Checks whether a number is even or odd using isEven API (https://isevenapi.xyz/)

Usage: iseven_api [OPTIONS] <NUMBER>

Arguments:
  <NUMBER>  Number to check

Options:
      --json  Print JSON response
  -h, --help  Print help
```

## Library

### Add dependency

```
cargo add iseven_api
```

### Library usage example

```rust
use std::error::Error;
use iseven_api::IsEvenApiClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialise the client
    let client = IsEvenApiClient::new();

    // Make requests
    let odd_num = client.get(41).await?;
    let even_num = client.get(42).await?;
    assert!(odd_num.isodd());
    assert!(even_num.iseven());

    Ok(())
}
```

### Documentation

<https://docs.rs/iseven_api/latest/iseven_api/>

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
