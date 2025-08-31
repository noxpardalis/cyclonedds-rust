mod error;
mod status;

pub use error::{Error, Result};
pub use status::Status;

#[cfg(feature = "internal")]
pub mod internal;
#[cfg(not(feature = "internal"))]
mod internal;

#[cfg(test)]
mod tests;
