pub mod cdr_bounds;
mod error;
mod topicable;

pub use cyclonedds_derive::Topicable;
pub use error::{Error, Result};
pub use topicable::Topicable;

#[cfg(feature = "internal")]
pub mod internal;
#[cfg(not(feature = "internal"))]
mod internal;

#[cfg(test)]
mod tests;
