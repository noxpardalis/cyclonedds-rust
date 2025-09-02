mod domain;
mod duration;
mod error;
pub mod state;
pub mod status;

pub use domain::Domain;
pub use duration::Duration;
pub use error::{Error, Result};
pub use state::State;
pub use status::Status;

#[cfg(feature = "internal")]
pub mod internal;
#[cfg(not(feature = "internal"))]
mod internal;

#[cfg(test)]
mod tests;
