use crate::Participant;
use crate::Result;

use crate::internal::ffi;

///
#[derive(Debug)]
pub struct Subscriber<'domain, 'participant> {
    pub(crate) inner: cyclonedds_sys::dds_entity_t,
    phantom: std::marker::PhantomData<&'participant Participant<'domain>>,
}

pub struct SubscriberBuilder<'domain, 'participant, 'qos> {
    participant: &'participant Participant<'domain>,
    qos: Option<&'qos crate::QoS>,
    listener: Option<crate::SubscriberListener>,
}

impl<'d, 'p, 'q> SubscriberBuilder<'d, 'p, 'q> {
    pub fn new(participant: &'p Participant<'d>) -> Self {
        Self {
            participant,
            qos: None,
            listener: None,
        }
    }

    pub fn with_qos(mut self, qos: &'q crate::QoS) -> Self {
        self.qos = Some(qos);
        self
    }

    pub fn with_listener(mut self, listener: crate::SubscriberListener) -> Self {
        self.listener = Some(listener);
        self
    }

    pub fn build(self) -> Result<Subscriber<'d, 'p>> {
        Ok(Subscriber {
            inner: ffi::dds_create_subscriber(
                self.participant.inner,
                self.qos.map(|qos| &qos.inner),
                self.listener
                    .map(|listener| listener.as_ffi())
                    .transpose()?
                    .as_ref(),
            )?,
            phantom: std::marker::PhantomData,
        })
    }
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
        Self::builder(participant).build()
    }

    ///
    pub fn builder<'q>(participant: &'p Participant<'d>) -> SubscriberBuilder<'d, 'p, 'q> {
        SubscriberBuilder::new(participant)
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

    ///
    pub fn set_listener<L>(&mut self, listener: L) -> Result<()>
    where
        L: AsRef<crate::SubscriberListener>,
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
        L: AsRef<crate::SubscriberListener>,
    {
        self.set_listener(listener).map(|_| self)
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

    #[test]
    fn test_subscriber_with_listener() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let participant = crate::Participant::new(&domain).unwrap();

        let listener = crate::SubscriberListener::new().with_data_on_readers(|_| ());

        let _ = Subscriber::new(&participant)
            .unwrap()
            .with_listener(&listener)
            .unwrap();

        let mut subscriber = Subscriber::new(&participant).unwrap();
        subscriber.set_listener(&listener).unwrap();
        subscriber.unset_listener().unwrap();
    }

    #[test]
    fn test_subscriber_with_listener_on_invalid_subscriber() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let participant = crate::Participant::new(&domain).unwrap();

        let listener = crate::SubscriberListener::new().with_data_on_readers(|_| ());

        let mut subscriber = Subscriber::new(&participant).unwrap();
        let subscriber_id = subscriber.inner;
        subscriber.inner = 0;
        let result = subscriber.set_listener(&listener).unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);
        let result = subscriber.unset_listener().unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);
        subscriber.inner = subscriber_id;
    }
}
