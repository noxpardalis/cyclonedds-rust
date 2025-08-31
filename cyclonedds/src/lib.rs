#![deny(unsafe_code)]

mod error;

pub use error::{Error, Result};

#[cfg(test)]
mod tests;
