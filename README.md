# IL_TZ

[![Rust](https://github.com/ofersadan85/il_tz/actions/workflows/rust.yml/badge.svg)](https://github.com/ofersadan85/il_tz/actions/workflows/rust.yml)

A Rust library for working with Israeli ID numbers (TZ is the Hebrew acronym for "ID number").

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
il_tz = "0.1.1"
```

Alternatively, run this command:

```bash
cargo add il_tz
```

## Example

This example shows how to convert an integer to a TZ, how to convert a string to a TZ, and validating them.

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
