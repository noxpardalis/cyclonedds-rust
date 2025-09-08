use crate::internal::ffi;
use crate::{Result, Subscriber, Topic};

///
#[derive(Debug)]
pub struct Reader<'domain, 'participant, 'topic, T>
where
    T: crate::Topicable,
{
    pub(crate) inner: cyclonedds_sys::dds_entity_t,
    phantom_topic: std::marker::PhantomData<&'topic Topic<'domain, 'participant, T>>,
}

pub struct ReaderBuilder<'domain, 'participant, 'topic, 'qos, T>
where
    T: crate::Topicable,
{
    subscriber: Option<&'participant Subscriber<'domain, 'participant>>,
    topic: &'topic Topic<'domain, 'participant, T>,
    qos: Option<&'qos crate::QoS>,
}

impl<'d, 'p, 't, 'q, T> ReaderBuilder<'d, 'p, 't, 'q, T>
where
    T: crate::Topicable,
{
    pub fn new(topic: &'t Topic<'d, 'p, T>) -> Self {
        Self {
            subscriber: None,
            topic,
            qos: None,
        }
    }

    pub fn with_qos(mut self, qos: &'q crate::QoS) -> Self {
        self.qos = Some(qos);
        self
    }

    pub fn with_subscriber(mut self, subscriber: &'p Subscriber<'d, 'p>) -> Self {
        self.subscriber = Some(subscriber);
        self
    }

    pub fn build(self) -> Result<Reader<'d, 'p, 't, T>> {
        Ok(Reader {
            inner: ffi::dds_create_reader(
                self.subscriber
                    .map(|subscriber| subscriber.inner)
                    .unwrap_or(ffi::dds_get_participant(self.topic.inner)?),
                self.topic.inner,
                self.qos.map(|qos| &qos.inner),
                None,
            )?,
            phantom_topic: std::marker::PhantomData,
        })
    }
}

impl<'d, 'p, 't, T> Reader<'d, 'p, 't, T>
where
    T: crate::Topicable,
{
    ///
    pub fn new(topic: &'t Topic<'d, 'p, T>) -> Result<Self> {
        Self::builder(topic).build()
    }

    ///
    pub fn builder<'q>(topic: &'t Topic<'d, 'p, T>) -> ReaderBuilder<'d, 'p, 't, 'q, T> {
        ReaderBuilder::new(topic)
    }

    ///
    pub fn take(&self) -> Result<Vec<crate::sample::SampleOrKey<T>>>
    where
        T: std::clone::Clone,
    {
        ffi::dds_take(self.inner)
    }

    ///
    pub fn read(&self) -> Result<Vec<crate::sample::SampleOrKey<T>>>
    where
        T: std::clone::Clone,
    {
        ffi::dds_read(self.inner)
    }

    ///
    pub fn peek(&self) -> Result<Vec<crate::sample::SampleOrKey<T>>>
    where
        T: std::clone::Clone,
    {
        ffi::dds_peek(self.inner)
    }

    ///
    pub fn matched_publications(&self) -> Result<Vec<crate::entity::InstanceHandle>> {
        let matched = ffi::dds_get_matched_publications(self.inner)?;
        let matched = matched
            .iter()
            .map(|&inner| crate::entity::InstanceHandle { inner })
            .collect();
        Ok(matched)
    }

    ///
    pub fn wait_for_historical_data(&self, timeout: crate::Duration) -> Result<()> {
        ffi::dds_reader_wait_for_historical_data(self.inner, timeout.inner)
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

impl<T> Drop for Reader<'_, '_, '_, T>
where
    T: crate::Topicable,
{
    fn drop(&mut self) {
        let result = ffi::dds_delete(self.inner);
        debug_assert!(result.is_ok());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_reader_create() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let qos = crate::QoS::new();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain).unwrap();
        let subscriber = crate::Subscriber::new(&participant).unwrap();
        let topic = Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();

        let _ = Reader::new(&topic).unwrap();
        let _ = Reader::builder(&topic).with_qos(&qos).build().unwrap();
        let _ = Reader::builder(&topic)
            .with_subscriber(&subscriber)
            .build()
            .unwrap();
        let _ = Reader::builder(&topic)
            .with_qos(&qos)
            .with_subscriber(&subscriber)
            .build()
            .unwrap();
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
        let result = Reader::new(&topic).unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);
        let result = Reader::builder(&topic).with_qos(&qos).build().unwrap_err();
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

        let reader = Reader::new(&topic).unwrap();
        let _ = reader.read().unwrap();
    }

    #[test]
    fn test_reader_empty_take() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain).unwrap();
        let topic = Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();

        let reader = Reader::new(&topic).unwrap();
        let _ = reader.take().unwrap();
    }

    #[test]
    fn test_reader_empty_peek() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain).unwrap();
        let topic = Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();

        let reader = Reader::new(&topic).unwrap();
        let _ = reader.peek().unwrap();
    }

    #[test]
    fn test_reader_create_from_existing() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain).unwrap();
        let topic = Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();

        let reader_01 = Reader::new(&topic).unwrap();
        let reader_02 = Reader::<crate::tests::topic::Data>::from_existing(reader_01.inner);
        assert_eq!(reader_01.inner, reader_02.inner);
    }

    #[test]
    fn test_reader_wait_for_historical_data() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain).unwrap();
        let topic = Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();

        let reader = Reader::new(&topic).unwrap();

        reader
            .wait_for_historical_data(crate::Duration::INFINITE)
            .unwrap();
    }

    #[test]
    fn test_reader_matched_publications() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain).unwrap();
        let topic = Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();

        let reader = Reader::new(&topic).unwrap();
        let writer = crate::Writer::new(&topic).unwrap();

        let matched = reader.matched_publications().unwrap();

        assert_eq!(matched.len(), 1);
        let expected = writer.instance_handle().unwrap();
        let actual = matched[0];
        assert_eq!(expected, actual);
    }
}
