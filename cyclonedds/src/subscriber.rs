use crate::Participant;
use crate::Result;
use crate::internal::ffi;

///
#[derive(Debug)]
pub struct Subscriber {
    pub(crate) inner: cyclonedds_sys::dds_entity_t,
}

///
#[derive(Debug)]
pub enum ParticipantOrSubscriber<'p> {
    ///
    Subscriber(&'p Subscriber),
    ///
    Participant(&'p Participant<'p>),
}

impl<'p> From<&'p Subscriber> for ParticipantOrSubscriber<'p> {
    fn from(subscriber: &'p Subscriber) -> Self {
        ParticipantOrSubscriber::Subscriber(subscriber)
    }
}

impl<'p> From<&'p Participant<'p>> for ParticipantOrSubscriber<'p> {
    fn from(participant: &'p Participant<'p>) -> Self {
        ParticipantOrSubscriber::Participant(participant)
    }
}

impl ParticipantOrSubscriber<'_> {
    pub(crate) fn inner(&self) -> cyclonedds_sys::dds_entity_t {
        match self {
            ParticipantOrSubscriber::Subscriber(subscriber) => subscriber.inner,
            ParticipantOrSubscriber::Participant(participant) => participant.inner,
        }
    }
}

impl<'d, 'p> Subscriber {
    ///
    pub fn new(participant: &'p Participant<'d>, _qos: Option<&crate::qos::QoS>) -> Result<Self> {
        Ok(Self {
            inner: ffi::dds_create_subscriber(participant.inner, None, None)?,
        })
    }
}

impl Drop for Subscriber {
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
        let participant = Participant::new(&domain, None).unwrap();
        let _ = Subscriber::new(&participant, None).unwrap();
    }

    #[test]
    fn test_subscriber_create_with_invalid_participant() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let mut participant = Participant::new(&domain, None).unwrap();
        let participant_id = participant.inner;
        participant.inner = 0;
        let result = Subscriber::new(&participant, None).unwrap_err();
        participant.inner = participant_id;

        assert_eq!(result, crate::Error::BadParameter);
    }

    #[test]
    fn test_participant_or_subscriber_create() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let participant = Participant::new(&domain, None).unwrap();
        let subscriber = Subscriber::new(&participant, None).unwrap();

        let participant_or_subscriber = ParticipantOrSubscriber::from(&participant);
        assert_eq!(participant_or_subscriber.inner(), participant.inner);

        let participant_or_subscriber = ParticipantOrSubscriber::from(&subscriber);
        assert_eq!(participant_or_subscriber.inner(), subscriber.inner);
    }
}
