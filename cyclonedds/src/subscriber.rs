use crate::Participant;
use crate::Result;

use crate::internal::ffi;

///
#[derive(Debug)]
pub struct Subscriber<'domain, 'participant> {
    pub(crate) inner: cyclonedds_sys::dds_entity_t,
    phantom: std::marker::PhantomData<&'participant Participant<'domain>>,
}

///
#[derive(Debug)]
pub enum ParticipantOrSubscriber<'d, 'p> {
    ///
    Subscriber(&'p Subscriber<'d, 'p>),
    ///
    Participant(&'p Participant<'d>),
}

impl<'d, 'p> From<&'p Subscriber<'d, 'p>> for ParticipantOrSubscriber<'d, 'p> {
    fn from(subscriber: &'p Subscriber<'d, 'p>) -> Self {
        ParticipantOrSubscriber::Subscriber(subscriber)
    }
}

impl<'d, 'p> From<&'p Participant<'d>> for ParticipantOrSubscriber<'d, 'p> {
    fn from(participant: &'p Participant<'d>) -> Self {
        ParticipantOrSubscriber::Participant(participant)
    }
}

impl ParticipantOrSubscriber<'_, '_> {
    pub(crate) fn inner(&self) -> cyclonedds_sys::dds_entity_t {
        match self {
            ParticipantOrSubscriber::Subscriber(subscriber) => subscriber.inner,
            ParticipantOrSubscriber::Participant(participant) => participant.inner,
        }
    }
}

impl<'d, 'p> Subscriber<'d, 'p> {
    ///
    pub fn new(participant: &'p Participant<'d>) -> Result<Self> {
        Ok(Self {
            inner: ffi::dds_create_subscriber(participant.inner, None, None)?,
            phantom: std::marker::PhantomData,
        })
    }

    ///
    pub fn new_with_qos(participant: &'p Participant<'d>, qos: &crate::qos::QoS) -> Result<Self> {
        Ok(Self {
            inner: ffi::dds_create_subscriber(participant.inner, Some(&qos.inner), None)?,
            phantom: std::marker::PhantomData,
        })
    }

    ///
    pub(crate) const fn from_existing(
        inner: cyclonedds_sys::dds_entity_t,
    ) -> std::mem::ManuallyDrop<Self> {
        std::mem::ManuallyDrop::new(Self {
            inner,
            phantom: std::marker::PhantomData,
        })
    }
}

impl Drop for Subscriber<'_, '_> {
    fn drop(&mut self) {
        let result = ffi::dds_delete(self.inner);
        debug_assert!(result.is_ok());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subscriber_create() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let qos = crate::QoS::new();
        let participant = Participant::new(&domain).unwrap();
        let _ = Subscriber::new(&participant).unwrap();
        let _ = Subscriber::new_with_qos(&participant, &qos).unwrap();
    }

    #[test]
    fn test_subscriber_create_with_invalid_participant() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let qos = crate::QoS::new();
        let mut participant = Participant::new(&domain).unwrap();
        let participant_id = participant.inner;
        participant.inner = 0;
        let result = Subscriber::new(&participant).unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);
        let result = Subscriber::new_with_qos(&participant, &qos).unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);
        participant.inner = participant_id;
    }

    #[test]
    fn test_participant_or_subscriber_create() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let participant = Participant::new(&domain).unwrap();
        let subscriber = Subscriber::new(&participant).unwrap();

        let participant_or_subscriber = ParticipantOrSubscriber::from(&participant);
        assert_eq!(participant_or_subscriber.inner(), participant.inner);

        let participant_or_subscriber = ParticipantOrSubscriber::from(&subscriber);
        assert_eq!(participant_or_subscriber.inner(), subscriber.inner);
    }

    #[test]
    fn test_subscriber_from_existing_subscriber() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let participant = crate::Participant::new(&domain).unwrap();
        let subscriber = Subscriber::new(&participant).unwrap();

        let new_subscriber = Subscriber::from_existing(subscriber.inner);

        assert_eq!(new_subscriber.inner, subscriber.inner);
    }
}
