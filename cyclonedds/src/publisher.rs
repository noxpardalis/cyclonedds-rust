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

    pub fn set_listener<L>(&mut self, listener: L) -> Result<()>
    where
        L: AsRef<crate::PublisherListener>,
    {
        listener
            .as_ref()
            .as_ffi()
            .map(|listener| ffi::dds_set_listener(self.inner, Some(listener.inner)))
            .flatten()
    }

    ///
    pub fn unset_listener(&mut self) -> Result<()> {
        ffi::dds_set_listener(self.inner, None)?;
        Ok(())
    }

    ///
    pub fn with_listener<L>(mut self, listener: L) -> Result<Self>
    where
        L: AsRef<crate::PublisherListener>,
    {
        self.set_listener(listener).map(|_| self)
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
    fn test_publisher_with_listener() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let participant = crate::Participant::new(&domain).unwrap();

        let listener = crate::PublisherListener::new();

        let _ = Publisher::new(&participant)
            .unwrap()
            .with_listener(&listener)
            .unwrap();

        let mut publisher = Publisher::new(&participant).unwrap();
        publisher.set_listener(&listener).unwrap();
        publisher.unset_listener().unwrap();
    }

    #[test]
    fn test_publisher_with_listener_on_invalid_publisher() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let participant = crate::Participant::new(&domain).unwrap();

        let listener = crate::PublisherListener::new();

        let mut publisher = Publisher::new(&participant).unwrap();
        let publisher_id = publisher.inner;
        publisher.inner = 0;
        let result = publisher.set_listener(&listener).unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);
        let result = publisher.unset_listener().unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);
        publisher.inner = publisher_id;
    }
}
