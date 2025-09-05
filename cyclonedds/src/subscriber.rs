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
    pub fn new(participant: &'p Participant<'d>, qos: Option<&crate::qos::QoS>) -> Result<Self> {
        Ok(Self {
            inner: ffi::dds_create_subscriber(participant.inner, qos.map(|qos| &qos.inner), None)?,
            phantom: Default::default(),
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
