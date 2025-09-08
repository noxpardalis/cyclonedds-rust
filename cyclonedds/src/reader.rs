use crate::Result;
use crate::{ParticipantOrSubscriber, Topic};

use crate::internal::ffi;

///
#[derive(Debug)]
pub struct Reader<'domain, 'participant, 'topic, T> {
    pub(crate) inner: cyclonedds_sys::dds_entity_t,
    phantom_topic: std::marker::PhantomData<&'topic Topic<'domain, 'participant, T>>,
}

impl<'d, 'p, 't, T> Reader<'d, 'p, 't, T> {
    ///
    pub fn new<P>(participant_or_subscriber: P, topic: &'t Topic<'d, 'p, T>) -> Result<Self>
    where
        P: Into<ParticipantOrSubscriber<'d, 'p>>,
    {
        Ok(Self {
            inner: ffi::dds_create_reader(
                participant_or_subscriber.into().inner(),
                topic.inner,
                None,
                None,
            )?,
            phantom_topic: std::marker::PhantomData,
        })
    }

    ///
    pub fn new_with_qos<P>(
        participant_or_subscriber: P,
        topic: &'t Topic<'d, 'p, T>,
        qos: &crate::QoS,
    ) -> Result<Self>
    where
        P: Into<ParticipantOrSubscriber<'d, 'p>>,
    {
        Ok(Self {
            inner: ffi::dds_create_reader(
                participant_or_subscriber.into().inner(),
                topic.inner,
                Some(&qos.inner),
                None,
            )?,
            phantom_topic: std::marker::PhantomData,
        })
    }

    ///
    pub fn take(&self) -> Result<Vec<Result<T, ()>>>
    where
        T: std::clone::Clone,
    {
        ffi::dds_take(self.inner)
    }

    ///
    pub fn read(&self) -> Result<Vec<Result<T, ()>>>
    where
        T: std::clone::Clone,
    {
        ffi::dds_read(self.inner)
    }

    ///
    pub fn peek(&self) -> Result<Vec<Result<T, ()>>>
    where
        T: std::clone::Clone,
    {
        ffi::dds_peek(self.inner)
    }

    ///
    pub(crate) const fn from_existing(
        inner: cyclonedds_sys::dds_entity_t,
    ) -> std::mem::ManuallyDrop<Self> {
        std::mem::ManuallyDrop::new(Self {
            inner,
            phantom_topic: std::marker::PhantomData,
        })
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
        let qos = crate::QoS::new();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain).unwrap();
        let subscriber = crate::Subscriber::new(&participant).unwrap();
        let topic = Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();

        let _ = Reader::new(&participant, &topic).unwrap();
        let _ = Reader::new_with_qos(&participant, &topic, &qos).unwrap();
        let _ = Reader::new(&subscriber, &topic).unwrap();
        let _ = Reader::new_with_qos(&subscriber, &topic, &qos).unwrap();
    }

    #[test]
    fn test_reader_create_with_invalid_topic() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let qos = crate::QoS::new();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain).unwrap();
        let mut topic = Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();

        let topic_id = topic.inner;
        topic.inner = 0;
        let result = Reader::new(&participant, &topic).unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);
        let result = Reader::new_with_qos(&participant, &topic, &qos).unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);
        topic.inner = topic_id;
    }

    #[test]
    fn test_reader_empty_read() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain).unwrap();
        let topic = Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();

        let reader = Reader::new(&participant, &topic).unwrap();
        let _ = reader.read().unwrap();
    }

    #[test]
    fn test_reader_empty_take() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain).unwrap();
        let topic = Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();

        let reader = Reader::new(&participant, &topic).unwrap();
        let _ = reader.take().unwrap();
    }

    #[test]
    fn test_reader_empty_peek() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain).unwrap();
        let topic = Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();

        let reader = Reader::new(&participant, &topic).unwrap();
        let _ = reader.peek().unwrap();
    }
}
