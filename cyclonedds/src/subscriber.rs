use crate::internal::ffi;
use crate::{Participant, Result};

///
#[derive(Debug)]
pub struct Subscriber<'domain, 'participant> {
    pub(crate) inner: cyclonedds_sys::dds_entity_t,
    phantom: std::marker::PhantomData<&'participant Participant<'domain>>,
}

///
pub struct SubscriberBuilder<'domain, 'participant, 'qos> {
    participant: &'participant Participant<'domain>,
    qos: Option<&'qos crate::QoS>,
}

impl<'d, 'p, 'q> SubscriberBuilder<'d, 'p, 'q> {
    ///
    pub fn new(participant: &'p Participant<'d>) -> Self {
        Self {
            participant,
            qos: None,
        }
    }

    ///
    pub fn with_qos(mut self, qos: &'q crate::QoS) -> Self {
        self.qos = Some(qos);
        self
    }

    ///
    pub fn build(self) -> Result<Subscriber<'d, 'p>> {
        Ok(Subscriber {
            inner: ffi::dds_create_subscriber(
                self.participant.inner,
                self.qos.map(|qos| &qos.inner),
                None,
            )?,
            phantom: std::marker::PhantomData,
        })
    }
}

impl<'d, 'p> Subscriber<'d, 'p> {
    ///
    pub fn new(participant: &'p Participant<'d>) -> Result<Self> {
        Self::builder(participant).build()
    }

    ///
    pub fn builder<'q>(participant: &'p Participant<'d>) -> SubscriberBuilder<'d, 'p, 'q> {
        SubscriberBuilder::new(participant)
    }

    ///
    pub fn notify_readers(&self) -> Result<()> {
        ffi::dds_notify_readers(self.inner)
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
        let _ = Subscriber::builder(&participant)
            .with_qos(&qos)
            .build()
            .unwrap();
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
        let result = Subscriber::builder(&participant)
            .with_qos(&qos)
            .build()
            .unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);
        participant.inner = participant_id;
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
