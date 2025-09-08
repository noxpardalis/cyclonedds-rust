use crate::internal::ffi;
use crate::{ParticipantOrSubscriber, Result, Topic};

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
    participant_or_subscriber: Option<ParticipantOrSubscriber<'domain, 'participant>>,
    topic: &'topic Topic<'domain, 'participant, T>,
    qos: Option<&'qos crate::QoS>,
}

impl<'d, 'p, 't, 'q, T> ReaderBuilder<'d, 'p, 't, 'q, T>
where
    T: crate::Topicable,
{
    pub fn new(topic: &'t Topic<'d, 'p, T>) -> Self {
        Self {
            participant_or_subscriber: None,
            topic,
            qos: None,
        }
    }

    pub fn with_qos(mut self, qos: &'q crate::QoS) -> Self {
        self.qos = Some(qos);
        self
    }

    pub fn with_participant_or_subscriber<P>(mut self, participant_or_subscriber: P) -> Self
    where
        P: Into<ParticipantOrSubscriber<'d, 'p>>,
    {
        self.participant_or_subscriber = Some(participant_or_subscriber.into());
        self
    }

    pub fn build(self) -> Result<Reader<'d, 'p, 't, T>> {
        Ok(Reader {
            inner: ffi::dds_create_reader(
                self.participant_or_subscriber
                    .map(|participant_or_subscriber| participant_or_subscriber.inner())
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
            .with_participant_or_subscriber(&subscriber)
            .build()
            .unwrap();
        let _ = Reader::builder(&topic)
            .with_qos(&qos)
            .with_participant_or_subscriber(&subscriber)
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
}
