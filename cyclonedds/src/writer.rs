use crate::internal::ffi;
use crate::{ParticipantOrPublisher, Result, Topic};

///
#[derive(Debug)]
pub struct Writer<'domain, 'participant, 'topic, T>
where
    T: crate::Topicable,
{
    pub(crate) inner: cyclonedds_sys::dds_entity_t,
    phantom_topic: std::marker::PhantomData<&'topic Topic<'domain, 'participant, T>>,
}

pub struct WriterBuilder<'domain, 'participant, 'topic, 'qos, T>
where
    T: crate::Topicable,
{
    participant_or_publisher: Option<ParticipantOrPublisher<'domain, 'participant>>,
    topic: &'topic Topic<'domain, 'participant, T>,
    qos: Option<&'qos crate::QoS>,
}

impl<'d, 'p, 't, 'q, T> WriterBuilder<'d, 'p, 't, 'q, T>
where
    T: crate::Topicable,
{
    pub fn new(topic: &'t Topic<'d, 'p, T>) -> Self {
        Self {
            participant_or_publisher: None,
            topic,
            qos: None,
        }
    }

    pub fn with_qos(mut self, qos: &'q crate::QoS) -> Self {
        self.qos = Some(qos);
        self
    }

    pub fn with_participant_or_publisher<P>(mut self, participant_or_publisher: P) -> Self
    where
        P: Into<ParticipantOrPublisher<'d, 'p>>,
    {
        self.participant_or_publisher = Some(participant_or_publisher.into());
        self
    }

    pub fn build(self) -> Result<Writer<'d, 'p, 't, T>> {
        Ok(Writer {
            inner: ffi::dds_create_writer(
                self.participant_or_publisher
                    .map(|participant_or_publisher| participant_or_publisher.inner())
                    .unwrap_or(ffi::dds_get_participant(self.topic.inner)?),
                self.topic.inner,
                self.qos.map(|qos| &qos.inner),
                None,
            )?,
            phantom_topic: std::marker::PhantomData,
        })
    }
}

impl<'d, 'p, 't, T> Writer<'d, 'p, 't, T>
where
    T: crate::Topicable,
{
    ///
    pub fn new(topic: &'t Topic<'d, 'p, T>) -> Result<Self> {
        Self::builder(topic).build()
    }

    ///
    pub fn builder<'q>(topic: &'t Topic<'d, 'p, T>) -> WriterBuilder<'d, 'p, 't, 'q, T> {
        WriterBuilder::new(topic)
    }

    ///
    pub fn write(&self, sample: &T) -> Result<()> {
        ffi::dds_write(self.inner, sample)?;
        Ok(())
    }

    ///
    pub fn write_with_timestamp(&self, sample: &T, timestamp: crate::Time) -> Result<()> {
        ffi::dds_write_with_timestamp(self.inner, sample, timestamp.inner)?;
        Ok(())
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

impl<T> Drop for Writer<'_, '_, '_, T>
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
    fn test_writer_create() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let qos = crate::QoS::new();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain).unwrap();
        let publisher = crate::Publisher::new(&participant).unwrap();
        let topic = Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();

        let _ = Writer::new(&topic).unwrap();
        let _ = Writer::builder(&topic).with_qos(&qos).build().unwrap();
        let _ = Writer::builder(&topic)
            .with_participant_or_publisher(&publisher)
            .build()
            .unwrap();
        let _ = Writer::builder(&topic)
            .with_participant_or_publisher(&publisher)
            .with_qos(&qos)
            .build()
            .unwrap();
    }

    #[test]
    fn test_writer_create_with_invalid_topic() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let qos = crate::QoS::new();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain).unwrap();
        let mut topic = Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();

        let topic_id = topic.inner;
        topic.inner = 0;
        let result = Writer::new(&topic).unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);
        let result = Writer::builder(&topic).with_qos(&qos).build().unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);
        topic.inner = topic_id;
    }

    #[test]
    fn test_writer_write() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain).unwrap();
        let topic = Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();
        let writer = Writer::new(&topic).unwrap();
        writer.write(&Default::default()).unwrap();
    }

    #[test]
    fn test_writer_write_with_timestamp() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain).unwrap();
        let topic = Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();
        let writer = Writer::new(&topic).unwrap();
        let timestamp = crate::Time::from_nanos(10001);
        writer
            .write_with_timestamp(&Default::default(), timestamp)
            .unwrap();
    }

    #[test]
    fn test_writer_write_on_invalid_writer() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain).unwrap();
        let topic = Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();
        let mut writer = Writer::new(&topic).unwrap();

        let writer_id = writer.inner;
        writer.inner = 0;
        let result = writer.write(&Default::default()).unwrap_err();
        writer.inner = writer_id;

        assert_eq!(result, crate::Error::BadParameter);
    }

    #[test]
    fn test_writer_write_with_timestamp_on_invalid_writer() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain).unwrap();
        let topic = Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();
        let mut writer = Writer::new(&topic).unwrap();

        let writer_id = writer.inner;
        writer.inner = 0;
        let timestamp = crate::Time::from_nanos(10001);
        let result = writer
            .write_with_timestamp(&Default::default(), timestamp)
            .unwrap_err();
        writer.inner = writer_id;

        assert_eq!(result, crate::Error::BadParameter);
    }
}
