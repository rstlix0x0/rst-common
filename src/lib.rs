//! A `rst-common` library is a simple Rust library that aggregate 
//! all of common crates, such as `serde`, `serde_json`, etc. 
//! 

#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

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

#[cfg(feature = "with-tokio")]
pub mod with_tokio {
    pub use tokio;
}

#[cfg(feature = "with-tracing")]
pub mod with_tracing {
    pub use tracing;
    pub use tracing_subscriber;
}

#[cfg(feature = "with-logging")]
pub mod with_logging {
    pub use env_logger;
    pub use log;
}

#[cfg(feature = "with-errors")]
pub mod with_errors {
    pub use anyhow;
    pub use thiserror;
}

#[cfg(feature = "with-tests")]
pub mod with_tests {
    pub use mockall;
    pub use table_test;
}

#[cfg(feature = "with-http-tokio")]
pub mod with_http_tokio {
    pub use axum;
    pub use hyper;
    pub use hyper_util;
    pub use tower;
    pub use tower_http;
}

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
