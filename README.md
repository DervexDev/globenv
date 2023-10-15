# globenv
Globally set and read environment variables (and not just for the current process) on Windows, macOS and Linux.

[![Version badge](https://img.shields.io/crates/v/globenv.svg)](https://crates.io/crates/globenv)
[![Downloads badge](https://img.shields.io/crates/d/globenv.svg)](https://crates.io/crates/globenv)
[![License badge](https://img.shields.io/crates/l/globenv.svg)](https://crates.io/crates/globenv)

## Example:
```rust
use globenv::{set_var, get_var};
// Set variable
set_var("key", "value").unwrap();
// Remove variable
set_var("key", "").unwrap();
// Read variable
get_var("key").unwrap();
```

## Credit
Based on the [globalenv](https://github.com/nicolasbauw/globalenv) by [@nicolasbauw](https://github.com/nicolasbauw).
