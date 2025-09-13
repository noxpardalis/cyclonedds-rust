//!

use crate::internal::ffi;
use crate::{Result, Status};

///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct InstanceHandle {
    pub(crate) inner: cyclonedds_sys::dds_instance_handle_t,
}

///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct EntityId {
    pub(crate) inner: cyclonedds_sys::dds_entity_t,
}

///
pub trait Entity {
    ///
    fn id(&self) -> EntityId;

    ///
    fn instance_handle(&self) -> Result<InstanceHandle> {
        let entity = self.id();
        let inner = ffi::dds_get_instance_handle(entity.inner)?;
        Ok(InstanceHandle { inner })
    }

    ///
    fn status_changes(&self) -> Result<Status> {
        let entity = self.id();
        let status = ffi::dds_get_status_changes(entity.inner)?;
        Status::from_bits(status).ok_or(crate::error::Error::BadParameter)
    }

    ///
    fn take_status(&self, mask: Option<Status>) -> Result<Status> {
        let entity = self.id();
        let mask = mask.unwrap_or(Status::all()).bits();
        let status = ffi::dds_take_status(entity.inner, mask)?;
        Status::from_bits(status).ok_or(crate::error::Error::BadParameter)
    }

    ///
    fn read_status(&self, mask: Option<Status>) -> Result<Status> {
        let entity = self.id();
        let mask = mask.unwrap_or(Status::all()).bits();
        let status = ffi::dds_read_status(entity.inner, mask)?;
        Status::from_bits(status).ok_or(crate::error::Error::BadParameter)
    }

    ///
    fn status_mask(&self) -> Result<Status> {
        let entity = self.id();
        let mask = ffi::dds_get_status_mask(entity.inner)?;
        Status::from_bits(mask).ok_or(crate::error::Error::BadParameter)
    }

    ///
    fn set_status_mask(&self, mask: Status) -> Result<()> {
        let entity = self.id();
        let mask = mask.bits();
        ffi::dds_set_status_mask(entity.inner, mask)
    }
}

macro_rules! impl_entity {
    ($ty:ty) => {
        impl Entity for $ty {
            fn id(&self) -> EntityId {
                EntityId { inner: self.inner }
            }
        }
    };
    ($ty:ty where $($bounds:tt)*) => {
        impl<$($bounds)*> Entity for $ty {
            fn id(&self) -> EntityId {
                EntityId { inner: self.inner }
            }
        }
    };
}

impl_entity!(crate::Participant<'_>);
impl_entity!(crate::Topic<'_, '_, T> where T: crate::Topicable);
impl_entity!(crate::Publisher<'_, '_>);
impl_entity!(crate::Subscriber<'_, '_>);
impl_entity!(crate::Reader<'_, '_, '_, T> where T: crate::Topicable);
impl_entity!(crate::Writer<'_, '_, '_, T> where T: crate::Topicable);
impl_entity!(crate::ReadCondition<'_, '_, '_, '_, T> where T: crate::Topicable);
impl_entity!(crate::QueryCondition<'_, '_, '_, '_, T, F> where T: crate::Topicable, F: Fn(&T) -> bool);
impl_entity!(crate::GuardCondition<'_>);
impl_entity!(crate::WaitSet<'_, '_, '_, A> where A);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_id_all_entity_types() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let participant = crate::Participant::new(&domain).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let topic =
            crate::Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();
        let publisher = crate::Publisher::new(&participant).unwrap();
        let subscriber = crate::Subscriber::new(&participant).unwrap();
        let reader = crate::Reader::new(&topic).unwrap();
        let writer = crate::Writer::new(&topic).unwrap();
        let read_condition = crate::ReadCondition::new(&reader, crate::state::sample::Any).unwrap();
        let query_condition =
            crate::QueryCondition::new(&reader, crate::State::empty(), |_| true).unwrap();
        let guard_condition = crate::GuardCondition::new(&participant).unwrap();
        let waitset = crate::WaitSet::<()>::new(&participant).unwrap();

        assert_eq!(participant.id().inner, participant.inner);
        assert_eq!(topic.id().inner, topic.inner);
        assert_eq!(publisher.id().inner, publisher.inner);
        assert_eq!(subscriber.id().inner, subscriber.inner);
        assert_eq!(reader.id().inner, reader.inner);
        assert_eq!(writer.id().inner, writer.inner);
        assert_eq!(read_condition.id().inner, read_condition.inner);
        assert_eq!(query_condition.id().inner, query_condition.inner);
        assert_eq!(guard_condition.id().inner, guard_condition.inner);
        assert_eq!(waitset.id().inner, waitset.inner);
    }

    #[test]
    fn test_entity_methods_on_invalid_participant() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let mut participant = crate::Participant::new(&domain).unwrap();
        let participant_id = participant.inner;
        participant.inner = 0;

        assert_eq!(
            crate::Error::BadParameter,
            participant.instance_handle().unwrap_err()
        );
        assert_eq!(
            crate::Error::BadParameter,
            participant.status_changes().unwrap_err()
        );
        assert_eq!(
            crate::Error::BadParameter,
            participant.take_status(None).unwrap_err()
        );
        assert_eq!(
            crate::Error::BadParameter,
            participant.read_status(None).unwrap_err()
        );
        assert_eq!(
            crate::Error::BadParameter,
            participant.status_mask().unwrap_err()
        );
        assert_eq!(
            crate::Error::BadParameter,
            participant
                .set_status_mask(crate::status::Status::InconsistentTopic)
                .unwrap_err()
        );

        participant.inner = participant_id;
    }

    #[test]
    fn test_entity_methods_on_participant() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let participant = crate::Participant::new(&domain).unwrap();

        let result = participant.instance_handle();
        assert!(result.is_ok());
        let status_changes = participant.status_changes().unwrap();
        assert!(status_changes.is_empty());
        let result = participant.set_status_mask(crate::Status::empty());
        assert!(result.is_ok());
        let mask = participant.status_mask().unwrap();
        assert_eq!(mask, crate::Status::empty());
        let status = participant
            .read_status(Some(crate::Status::empty()))
            .unwrap();
        assert!(status.is_empty());
        let status = participant
            .take_status(Some(crate::Status::empty()))
            .unwrap();
        assert!(status.is_empty());
    }

    #[test]
    fn test_entity_methods_on_reader() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain).unwrap();
        let topic =
            crate::Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();
        let reader = crate::Reader::new(&topic).unwrap();

        let result = reader.instance_handle();
        assert!(result.is_ok());
        let status_changes = reader.status_changes().unwrap();
        assert!(status_changes.is_empty());
        let result = reader.set_status_mask(crate::Status::SubscriptionMatched);
        assert!(result.is_ok());
        let mask = reader.status_mask().unwrap();
        assert_eq!(mask, crate::Status::SubscriptionMatched);
        let status = reader
            .read_status(Some(crate::Status::SubscriptionMatched))
            .unwrap();
        assert!(status.is_empty());
        let status = reader
            .take_status(Some(crate::Status::SubscriptionMatched))
            .unwrap();
        assert!(status.is_empty());
    }
}
