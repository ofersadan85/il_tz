# IL_TZ

[![Rust](https://github.com/ofersadan85/il_tz/actions/workflows/rust.yml/badge.svg)](https://github.com/ofersadan85/il_tz/actions/workflows/rust.yml)
[![Publish](https://github.com/ofersadan85/il_tz/actions/workflows/publish.yml/badge.svg)](https://github.com/ofersadan85/il_tz/actions/workflows/publish.yml)
[![Crates.io](https://img.shields.io/crates/v/il_tz.svg)](https://crates.io/crates/il_tz)
[![Docs.rs](https://docs.rs/il_tz/badge.svg)](https://docs.rs/il_tz)
[![License](https://img.shields.io/crates/l/il_tz.svg)](LICENSE.md)

A Rust library for working with Israeli ID numbers (TZ is the [Hebrew](https://en.wikipedia.org/wiki/Israeli_identity_card) acronym for "ID").

## Install (CLI)

Download the binary for your system from the [latest release](https://github.com/ofersadan85/il_tz/releases/latest)

You can also install it using `cargo install`:

```bash
cargo install il_tz --features cli
```

## Usage (CLI)

```text
> iltz --help
Validate Israeli ID numbers (TZ)

Usage: iltz.exe [TZ]... [COMMAND]

Commands:
  generate  Generates a list of valid TZ values
  help      Print this message or the help of the given subcommand(s)

Arguments:
  [TZ]...  A list of ID numbers (TZ) to validate

Options:
  -h, --help  Print help
```

### CLI Examples

```text
> iltz 999999998 999999999
999999998 true
999999999 false

> iltz generate 50 100
000000059
000000067
000000075
000000083
000000091
000000109
```

## As a dependency

To use it in your own Rust projects, run this command:

```bash
cargo add il_tz
```

Alternatively, add this to your Cargo.toml:

```toml
[dependencies]
il_tz = "0.1.3"
```

> **Warning**
> Verify the latest version number on [crates.io](https://crates.io/crates/il_tz).

## Rust Example

This example shows how to convert an integer to a `TZ`, how to convert a string to a `TZ`, and validating them.

```rust
use il_tz::{int2tz, str2tz, tz2str, validate};

fn main() {
    // A valid TZ example
    let valid = int2tz(999_999_998).unwrap();
    println!("{} {}", tz2str(&valid), validate(&valid));

    // An invalid TZ example
    let invalid = int2tz(999_999_999).unwrap();
    println!("{} {}", tz2str(&invalid), validate(&invalid));

    // Leading zeros can be used or omitted
    let tz1 = str2tz("000000141").unwrap();
    let tz2 = str2tz("00141").unwrap();
    let tz3 = int2tz(141).unwrap();
    assert_eq!(tz1, tz2);
    assert_eq!(tz1, tz3);
}
```

> **Note**
> This library does not validate the ID number (TZ) against any sort of population database, it only validates it mathematically.

## License / Warranty

This library is licensed under the MIT license (which means it's free to use, reuse, repackage etc.) See the LICENSE file for more information. This library is provided as-is, without any warranty. Use at your own risk.
