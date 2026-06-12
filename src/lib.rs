//! Invezgo SDK for Rust
//! Fully-typed and covering all endpoints 1:1.

pub mod client;
pub mod error;
pub mod models;
pub mod api;

pub use client::{InvezgoClient, InvezgoClientBuilder};
pub use error::InvezgoError;
