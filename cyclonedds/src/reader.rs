use crate::Result;
use crate::internal::ffi;
use crate::{ParticipantOrSubscriber, Topic};

///
#[derive(Debug)]
pub struct Reader<'topic, 'domain, 'participant, T> {
    pub(crate) inner: cyclonedds_sys::dds_entity_t,
    phantom_topic: std::marker::PhantomData<&'topic Topic<'domain, 'participant, T>>,
}

impl<'t, 'd, 'p, T> Reader<'t, 'd, 'p, T> {
    ///
    pub fn new<P>(
        participant_or_subscriber: P,
        topic: &'t Topic<'d, 'p, T>,
        qos: Option<&crate::QoS>,
    ) -> Result<Self>
    where
        P: Into<ParticipantOrSubscriber<'d, 'p>>,
    {
        Ok(Self {
            inner: ffi::dds_create_reader(
                participant_or_subscriber.into().inner(),
                topic.inner,
                qos.map(|qos| &qos.inner),
                None,
            )?,
            phantom_topic: Default::default(),
        })
    }

    ///
    pub fn take(&self) -> Result<Vec<T>>
    where
        T: std::clone::Clone,
    {
        ffi::dds_take(self.inner)
    }

    ///
    pub fn read(&self) -> Result<Vec<T>>
    where
        T: std::clone::Clone,
    {
        ffi::dds_read(self.inner)
    }

    ///
    pub fn peek(&self) -> Result<Vec<T>>
    where
        T: std::clone::Clone,
    {
        ffi::dds_peek(self.inner)
    }
}

impl<T> Drop for Reader<'_, '_, '_, T> {
    fn drop(&mut self) {
        let result = ffi::dds_delete(self.inner);
        debug_assert!(result.is_ok());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reader_create() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain, None).unwrap();
        let subscriber = crate::Subscriber::new(&participant, None).unwrap();
        let topic =
            Topic::<crate::tests::topic::Data>::new(&participant, &topic_name, None).unwrap();

        let _ = Reader::new(&participant, &topic, None).unwrap();
        let _ = Reader::new(&subscriber, &topic, None).unwrap();
    }

    #[test]
    fn test_reader_create_with_invalid_topic() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain, None).unwrap();
        let mut topic =
            Topic::<crate::tests::topic::Data>::new(&participant, &topic_name, None).unwrap();

        let topic_id = topic.inner;
        topic.inner = 0;
        let result = Reader::new(&participant, &topic, None).unwrap_err();
        topic.inner = topic_id;

        assert_eq!(result, crate::Error::BadParameter);
    }

    #[test]
    fn test_reader_empty_read() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain, None).unwrap();
        let topic =
            Topic::<crate::tests::topic::Data>::new(&participant, &topic_name, None).unwrap();

        let reader = Reader::new(&participant, &topic, None).unwrap();
        let _ = reader.read().unwrap();
    }

    #[test]
    fn test_reader_empty_take() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain, None).unwrap();
        let topic =
            Topic::<crate::tests::topic::Data>::new(&participant, &topic_name, None).unwrap();

        let reader = Reader::new(&participant, &topic, None).unwrap();
        let _ = reader.take().unwrap();
    }

    #[test]
    fn test_reader_empty_peek() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain, None).unwrap();
        let topic =
            Topic::<crate::tests::topic::Data>::new(&participant, &topic_name, None).unwrap();

        let reader = Reader::new(&participant, &topic, None).unwrap();
        let _ = reader.peek().unwrap();
    }
}
