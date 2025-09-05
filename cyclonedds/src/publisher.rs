use crate::internal::ffi;
use crate::{Participant, Result};

///
#[derive(Debug)]
pub struct Publisher<'domain, 'participant> {
    pub(crate) inner: cyclonedds_sys::dds_entity_t,
    phantom: std::marker::PhantomData<&'participant Participant<'domain>>,
}

pub struct PublisherBuilder<'domain, 'participant, 'qos> {
    participant: &'participant Participant<'domain>,
    qos: Option<&'qos crate::QoS>,
}

impl<'d, 'p, 'q> PublisherBuilder<'d, 'p, 'q> {
    pub fn new(participant: &'p Participant<'d>) -> Self {
        Self {
            participant,
            qos: None,
        }
    }

    pub fn with_qos(mut self, qos: &'q crate::QoS) -> Self {
        self.qos = Some(qos);
        self
    }

    pub fn build(self) -> Result<Publisher<'d, 'p>> {
        Ok(Publisher {
            inner: ffi::dds_create_publisher(
                self.participant.inner,
                self.qos.map(|qos| &qos.inner),
                None,
            )?,
            phantom: std::marker::PhantomData,
        })
    }
}

impl<'d, 'p> Publisher<'d, 'p> {
    ///
    pub fn new(participant: &'p Participant<'d>) -> Result<Self> {
        Self::builder(participant).build()
    }

    ///
    pub fn builder<'q>(participant: &'p Participant<'d>) -> PublisherBuilder<'d, 'p, 'q> {
        PublisherBuilder::new(participant)
    }

    ///
    pub fn suspend(&self) -> Result<()> {
        ffi::dds_suspend(self.inner)
    }

    ///
    pub fn resume(&self) -> Result<()> {
        ffi::dds_resume(self.inner)
    }

    ///
    pub fn wait_for_acks(&self, timeout: crate::Duration) -> Result<()> {
        ffi::dds_wait_for_acks(self.inner, timeout.inner)
    }

    ///
    #[allow(unused)]
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
        let _ = Publisher::builder(&participant)
            .with_qos(&qos)
            .build()
            .unwrap();
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
        let result = Publisher::builder(&participant)
            .with_qos(&qos)
            .build()
            .unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);
        participant.inner = participant_id;
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
