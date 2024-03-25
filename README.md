# rst-common

> A shortcut to common Rust crates. 

## Motivation

The reason behind this `crate` is due to a lot of common crate libraries that I need to install
for each of my projects and there are a lot of common libraries I've been used to. 

So I was just thinking that rather than I'm always
repeat my activities, such as finding the library, register it manually, and so on, I was thinking that it will be nice if I only need 
to install a single crate which contains all of my favorite crates.

## Installation

Standard usage :

```toml
[dependencies]
rst-common = {version = "1.3.0"}
```

Example install `with-tokio` feature

```toml
[dependencies]
rst-common = {version = "1.3.0", features: ["with-tokio"]}
```

## Examples

You can look at to `examples` directory to look at how to use this library.

Codes

```ignore
use rst_common::standard::uuid::Uuid;

fn main() {
    let uid = Uuid::new_v4();
    println!("{}", uid)
}
```

### Special Crates

> **WARNING**
>
> Special crates means, there are spesific ways to use, like `thiserror` and `serde`. 
> These crates need a "special" way to use because of their `proc_macro` usages.

> **WARNING**
>
> Remove package `mockall`

---

#### `thiserror`

```ignore
use rst_common::with_errors::thiserror:{self, Error};

#[derive(Error)]
enum MyError {
    #[error("general error")]
    GeneralError,
}
```

Need to use `self` which give the compiler a signal that the `Error` macro comes from the `thiserror` and the crate exists 

#### `serde`

```ignore
use rst_common::standard::serde::{self, Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
struct Message {
    msg: String,
}
```

Spesific for the `serde` crate, since this crate depends on `serde_derive` which call the `serde::`, we need to explicitly tell the `serde` the used crate path. 

## Version Requirements

All of used crates will follow these requirements

- All crates below of `1.x` (`<1.0.0`), will maintain the version until the minor version, example: `~0.1`, which mean, all new updated `patch` version should be automatically updated, but not for `minor` version. The version `~0.1` will automatically update to `0.1.1` and so on, as long as the `minor` version is equal. The reason behind this rule is, any version below of `1.0.0` assumed still under active development, and to maintain compatibility, it will be guarded using it's `minor` version.  
- All crates equal or greater than `1.x`, will maintain the version using it's major version, example: `~1`. For example, if we are using a crate that it's version: `1.0.1`, this rule should be updated automatically for the next version of: `1.0.2` or even `1.1.0`, but it will not follow the version `2.0.0` (assumed changed on `major` version will give us breaking compatibility)

> **INFO**
>
> If there are any updated versions to the supported crates, it will be updated manually through this library

For example, now we're support `tokio: ~1`, it means if the `tokio` library release it's new major version: `2.0.0`, this crate will updated manually to support latest versions

## Categories (Feature Flags)

- `standard`
- `full` 
- `with-tokio`
- `with-tracing`
- `with-logging` 
- `with-errors`
- `with-tests`
- `with-http-tokio`
- `with-cryptography`

This library actually is just re-export all common crates installed based on *features* you chose.

---

### `standard` feature 

```rust
#[cfg(feature = "standard")]
pub mod standard {
    pub use async_trait;
    pub use chrono;
    pub use dyn_clone;
    pub use erased_serde;
    pub use futures;
    pub use serde;
    pub use serde_json;
    pub use uuid;
}
```

### `full` feature

```rust
#[cfg(feature = "full")]
pub mod full {
    pub use async_trait;
    pub use chrono;
    pub use dyn_clone;
    pub use erased_serde;
    pub use futures;
    pub use serde;
    pub use serde_json;
    pub use uuid;
    pub use tokio;
    pub use tracing;
    pub use tracing_subscriber;
    pub use env_logger;
    pub use log;
    pub use anyhow;
    pub use thiserror;
    pub use mockall;
    pub use table_test;
    pub use axum;
    pub use hyper;
    pub use hyper_util;
    pub use tower;
    pub use tower_http;
    pub use rand;
    pub use rand_chacha;
    pub use hex;
    pub use chacha20poly1305;
    pub use blake3;
    pub use argon2;
    pub use ring;
    pub use ed25519_dalek;
    pub use x25519_dalek;
}
```

### `with-tokio` feature 

```rust
#[cfg(feature = "with-tokio")]
pub mod with_tokio {
    pub use tokio;
}
```

### `with-tracing` feature

```rust
#[cfg(feature = "with-tracing")]
pub mod with_tracing {
    pub use tracing;
    pub use tracing_subscriber;
}
```

### `with-logging` feature

```rust
#[cfg(feature = "with-logging")]
pub mod with_logging {
    pub use env_logger;
    pub use log;
}
```

### `with-errors` feature

```rust
#[cfg(feature = "with-errors")]
pub mod with_errors {
    pub use anyhow;
    pub use thiserror;
}
```

### `with-tests` feature

```rust
#[cfg(feature = "with-tests")]
pub mod with_tests {
    pub use table_test;
}
```

### `with-http-tokio` feature

```rust
#[cfg(feature = "with-http-tokio")]
pub mod with_http_tokio {
    pub use axum;
    pub use hyper;
    pub use hyper_util;
    pub use tower;
    pub use tower_http;
}
```

### `with-cryptography` feature

```rust
#[cfg(feature = "with-cryptography")]
pub mod with_cryptography {
    pub use rand;
    pub use rand_chacha;
    pub use hex;
    pub use chacha20poly1305;
    pub use blake3;
    pub use argon2;
    pub use ring;
    pub use sha2;
    pub use ed25519_dalek;
    pub use x25519_dalek;
}
```