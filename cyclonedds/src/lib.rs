#![deny(unsafe_code)]

pub mod cdr_bounds;
mod duration;
mod error;
pub mod state;
mod time;
mod topicable;

pub use duration::Duration;
pub use error::{Error, Result};
pub use state::State;
pub use time::Time;
pub use topicable::Topicable;

#[cfg(feature = "internal")]
pub mod internal;
#[cfg(not(feature = "internal"))]
mod internal;

#[cfg(test)]
mod tests;
