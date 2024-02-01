# globenv

Globally set & read environment variables and paths (not just for the current process) on Windows, macOS or Linux

<div>
  <a href="https://crates.io/crates/globenv"><img alt='Version badge' src='https://img.shields.io/crates/v/globenv.svg'></a>
  <a href="https://crates.io/crates/globenv"><img alt='Downloads badge' src='https://img.shields.io/crates/d/globenv.svg'></a>
  <a href="https://crates.io/crates/globenv"><img alt='License badge' src='https://img.shields.io/crates/l/globenv.svg'></a>
  <a href="https://docs.rs/globenv"><img alt="Docs badge" src="https://img.shields.io/docsrs/globenv"></a>
</div>

## Example:

```rust
use globenv::*;

// Get environment variable
get_var("key").unwrap().unwrap();
// Set environment variable
set_var("key", "value").unwrap();
// Remove environment variable
remove_var("key").unwrap();

// Get all environment paths
get_paths().unwrap();
// Set environment path
set_path("example/path").unwrap();
// Remove environment path
remove_path("example/path").unwrap();
```

## Credit:

Based on the [globalenv](https://github.com/nicolasbauw/globalenv) by [@nicolasbauw](https://github.com/nicolasbauw)
