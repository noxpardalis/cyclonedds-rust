#![deny(unsafe_code)]

pub mod cdr_bounds;
mod domain;
mod duration;
pub mod entity;
mod error;
mod guard_condition;
pub mod listener;
mod participant;
mod publisher;
pub mod qos;
mod query_condition;
mod read_condition;
mod reader;
pub mod sample;
pub mod state;
pub mod status;
mod subscriber;
mod time;
mod topic;
mod topicable;
mod waitset;
mod writer;

pub use cyclonedds_derive::Topicable;
pub use domain::Domain;
pub use duration::Duration;
pub use error::{Error, Result};
pub use guard_condition::GuardCondition;
pub use listener::{
    Listener, PublisherListener, ReaderListener, SubscriberListener, TopicListener, WriterListener,
};
pub use participant::Participant;
pub use publisher::Publisher;
pub use qos::QoS;
pub use query_condition::QueryCondition;
pub use read_condition::ReadCondition;
pub use reader::Reader;
pub use state::State;
pub use status::Status;
pub use subscriber::Subscriber;
pub use time::Time;
pub use topic::Topic;
pub use topicable::Topicable;
pub use waitset::WaitSet;
pub use writer::Writer;

#[cfg(feature = "internal")]
pub mod internal;
#[cfg(not(feature = "internal"))]
mod internal;

#[cfg(test)]
mod tests;
