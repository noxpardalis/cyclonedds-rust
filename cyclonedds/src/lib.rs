#![deny(unsafe_code)]

pub mod cdr_bounds;
mod domain;
mod duration;
pub mod entity;
mod error;
mod participant;
pub mod qos;
pub mod sample;
pub mod state;
pub mod status;
mod time;
mod topicable;

pub use domain::Domain;
pub use duration::Duration;
pub use error::{Error, Result};
pub use participant::Participant;
pub use qos::QoS;
pub use state::State;
pub use status::Status;
pub use time::Time;
pub use topicable::Topicable;

#[cfg(feature = "internal")]
pub mod internal;
#[cfg(not(feature = "internal"))]
mod internal;

#[cfg(test)]
mod tests;
