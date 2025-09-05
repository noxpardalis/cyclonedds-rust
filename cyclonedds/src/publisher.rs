use crate::Participant;
use crate::Result;

use crate::internal::ffi;

///
#[derive(Debug)]
pub struct Publisher<'domain, 'participant> {
    pub(crate) inner: cyclonedds_sys::dds_entity_t,
    phantom: std::marker::PhantomData<&'participant Participant<'domain>>,
}

///
#[derive(Debug)]
pub enum ParticipantOrPublisher<'d, 'p> {
    ///
    Publisher(&'p Publisher<'d, 'p>),
    ///
    Participant(&'p Participant<'d>),
}

impl<'d, 'p> From<&'p Publisher<'d, 'p>> for ParticipantOrPublisher<'d, 'p> {
    fn from(publisher: &'p Publisher<'d, 'p>) -> Self {
        ParticipantOrPublisher::Publisher(publisher)
    }
}

impl<'d, 'p> From<&'p Participant<'d>> for ParticipantOrPublisher<'d, 'p> {
    fn from(participant: &'p Participant<'d>) -> Self {
        ParticipantOrPublisher::Participant(participant)
    }
}

impl ParticipantOrPublisher<'_, '_> {
    pub(crate) fn inner(&self) -> cyclonedds_sys::dds_entity_t {
        match self {
            ParticipantOrPublisher::Publisher(publisher) => publisher.inner,
            ParticipantOrPublisher::Participant(participant) => participant.inner,
        }
    }
}

impl<'d, 'p> Publisher<'d, 'p> {
    ///
    pub fn new(participant: &'p Participant<'d>) -> Result<Self> {
        Ok(Self {
            inner: ffi::dds_create_publisher(participant.inner, None, None)?,
            phantom: std::marker::PhantomData,
        })
    }

    ///
    pub fn new_with_qos(participant: &'p Participant<'d>, qos: &crate::qos::QoS) -> Result<Self> {
        Ok(Self {
            inner: ffi::dds_create_publisher(participant.inner, Some(&qos.inner), None)?,
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

impl Drop for Publisher<'_, '_> {
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
        let qos = crate::QoS::new();
        let participant = Participant::new(&domain).unwrap();
        let _ = Publisher::new(&participant).unwrap();
        let _ = Publisher::new_with_qos(&participant, &qos).unwrap();
    }

    #[test]
    fn test_publisher_create_with_invalid_participant() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let qos = crate::QoS::new();
        let mut participant = Participant::new(&domain).unwrap();
        let participant_id = participant.inner;
        participant.inner = 0;
        let result = Publisher::new(&participant).unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);
        let result = Publisher::new_with_qos(&participant, &qos).unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);
        participant.inner = participant_id;
    }

    #[test]
    fn test_participant_or_publisher_create() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let participant = Participant::new(&domain).unwrap();
        let publisher = Publisher::new(&participant).unwrap();

        let participant_or_publisher = ParticipantOrPublisher::from(&participant);
        assert_eq!(participant_or_publisher.inner(), participant.inner);

        let participant_or_publisher = ParticipantOrPublisher::from(&publisher);
        assert_eq!(participant_or_publisher.inner(), publisher.inner);
    }

    #[test]
    fn test_publisher_from_existing_publisher() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let participant = crate::Participant::new(&domain).unwrap();
        let publisher = Publisher::new(&participant).unwrap();

        let new_publisher = Publisher::from_existing(publisher.inner);

        assert_eq!(new_publisher.inner, publisher.inner);
    }
}
