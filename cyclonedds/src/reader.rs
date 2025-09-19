use crate::Result;
use crate::internal::ffi;
use crate::{ParticipantOrSubscriber, Topic};

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
            phantom_topic: Default::default(),
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
            phantom_topic: Default::default(),
        })
    }

    ///
    pub fn take(&self) -> Result<Vec<Result<crate::sample::Sample<T>, crate::sample::Info>>>
    where
        T: std::clone::Clone,
    {
        ffi::dds_take(self.inner)
    }

    ///
    pub fn read(&self) -> Result<Vec<Result<crate::sample::Sample<T>, crate::sample::Info>>>
    where
        T: std::clone::Clone,
    {
        ffi::dds_read(self.inner)
    }

    ///
    pub fn peek(&self) -> Result<Vec<Result<crate::sample::Sample<T>, crate::sample::Info>>>
    where
        T: std::clone::Clone,
    {
        ffi::dds_peek(self.inner)
    }

    ///
    pub fn set_listener<L>(&mut self, listener: L) -> Result<()>
    where
        T: serde::ser::Serialize + serde::de::DeserializeOwned + std::clone::Clone + Default,
        L: AsRef<crate::ReaderListener<T>>,
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
        T: serde::ser::Serialize + serde::de::DeserializeOwned + std::clone::Clone + Default,
        L: AsRef<crate::ReaderListener<T>>,
    {
        self.set_listener(listener).map(|_| self)
    }

    ///
    pub(crate) fn from_existing(
        inner: cyclonedds_sys::dds_entity_t,
    ) -> std::mem::ManuallyDrop<Self> {
        std::mem::ManuallyDrop::new(Self {
            inner,
            phantom_topic: Default::default(),
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

    #[test]
    fn test_reader_with_listener() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain).unwrap();
        let topic = Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();

        let listener = crate::ReaderListener::new()
            .with_data_available(|_| unreachable!())
            .with_liveliness_changed(|_, _| unreachable!())
            .with_requested_deadline_missed(|_, _| unreachable!())
            .with_requested_incompatible_qos(|_, _| unreachable!())
            .with_sample_lost(|_, _| unreachable!())
            .with_sample_rejected(|_, _| unreachable!())
            .with_subscription_matched(|_, _| unreachable!());

        let _ = Reader::new(&participant, &topic)
            .unwrap()
            .with_listener(&listener)
            .unwrap();

        let mut reader = Reader::new(&participant, &topic).unwrap();
        reader.set_listener(&listener).unwrap();
        reader.unset_listener().unwrap();
    }

    #[test]
    fn test_reader_with_listener_on_invalid_reader() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain).unwrap();
        let topic = Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();

        let listener = crate::ReaderListener::new().with_data_available(|_| unreachable!());

        let mut reader = Reader::new(&participant, &topic).unwrap();
        let reader_id = reader.inner;
        reader.inner = 0;
        let result = reader.set_listener(&listener).unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);
        let result = reader.unset_listener().unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);
        reader.inner = reader_id;
    }

    #[test]
    fn test_reader_create_from_existing() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain).unwrap();
        let topic = Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();

        let reader_01 = Reader::new(&participant, &topic).unwrap();
        let reader_02 = Reader::<crate::tests::topic::Data>::from_existing(reader_01.inner);
        assert_eq!(reader_01.inner, reader_02.inner);
    }
}
