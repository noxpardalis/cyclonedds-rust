use crate::Participant;
use crate::Result;
use crate::internal::ffi;

///
#[derive(Debug)]
pub struct Publisher {
    pub(crate) inner: cyclonedds_sys::dds_entity_t,
}

///
#[derive(Debug)]
pub enum ParticipantOrPublisher<'p> {
    ///
    Publisher(&'p Publisher),
    ///
    Participant(&'p Participant<'p>),
}

impl<'p> From<&'p Publisher> for ParticipantOrPublisher<'p> {
    fn from(publisher: &'p Publisher) -> Self {
        ParticipantOrPublisher::Publisher(publisher)
    }
}

impl<'p> From<&'p Participant<'p>> for ParticipantOrPublisher<'p> {
    fn from(participant: &'p Participant<'p>) -> Self {
        ParticipantOrPublisher::Participant(participant)
    }
}

impl ParticipantOrPublisher<'_> {
    pub(crate) fn inner(&self) -> cyclonedds_sys::dds_entity_t {
        match self {
            ParticipantOrPublisher::Publisher(publisher) => publisher.inner,
            ParticipantOrPublisher::Participant(participant) => participant.inner,
        }
    }
}

impl<'d, 'p> Publisher {
    ///
    pub fn new(participant: &'p Participant<'d>, _qos: Option<&crate::qos::QoS>) -> Result<Self> {
        Ok(Self {
            inner: ffi::dds_create_publisher(participant.inner, None, None)?,
        })
    }
}

impl Drop for Publisher {
    fn drop(&mut self) {
        let result = ffi::dds_delete(self.inner);
        debug_assert!(result.is_ok());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_publisher_create() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let participant = Participant::new(&domain, None).unwrap();
        let _ = Publisher::new(&participant, None).unwrap();
    }

    #[test]
    fn test_publisher_create_with_invalid_participant() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let mut participant = Participant::new(&domain, None).unwrap();
        let participant_id = participant.inner;
        participant.inner = 0;
        let result = Publisher::new(&participant, None).unwrap_err();
        participant.inner = participant_id;

        assert_eq!(result, crate::Error::BadParameter);
    }

    #[test]
    fn test_participant_or_publisher_create() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let participant = Participant::new(&domain, None).unwrap();
        let publisher = Publisher::new(&participant, None).unwrap();

        let participant_or_publisher = ParticipantOrPublisher::from(&participant);
        assert_eq!(participant_or_publisher.inner(), participant.inner);

        let participant_or_publisher = ParticipantOrPublisher::from(&publisher);
        assert_eq!(participant_or_publisher.inner(), publisher.inner);
    }
}
