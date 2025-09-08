use crate::Result;
use crate::internal::ffi;
use crate::{ParticipantOrPublisher, Topic};

///
#[derive(Debug)]
pub struct Writer<'domain, 'participant, 'topic, T> {
    pub(crate) inner: cyclonedds_sys::dds_entity_t,
    phantom_topic: std::marker::PhantomData<&'topic Topic<'domain, 'participant, T>>,
}

impl<'d, 'p, 't, T> Writer<'d, 'p, 't, T> {
    ///
    pub fn new<P>(participant_or_publisher: P, topic: &'t Topic<'d, 'p, T>) -> Result<Self>
    where
        P: Into<ParticipantOrPublisher<'d, 'p>>,
    {
        Ok(Self {
            inner: ffi::dds_create_writer(
                participant_or_publisher.into().inner(),
                topic.inner,
                None,
                None,
            )?,
            phantom_topic: Default::default(),
        })
    }

    ///
    pub fn new_with_qos<P>(
        participant_or_publisher: P,
        topic: &'t Topic<'d, 'p, T>,
        qos: &crate::QoS,
    ) -> Result<Self>
    where
        P: Into<ParticipantOrPublisher<'d, 'p>>,
    {
        Ok(Self {
            inner: ffi::dds_create_writer(
                participant_or_publisher.into().inner(),
                topic.inner,
                Some(&qos.inner),
                None,
            )?,
            phantom_topic: Default::default(),
        })
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
    pub(crate) fn from_existing(
        inner: cyclonedds_sys::dds_entity_t,
    ) -> std::mem::ManuallyDrop<Self> {
        std::mem::ManuallyDrop::new(Self {
            inner,
            phantom_topic: Default::default(),
        })
    }
}

impl<T> Drop for Writer<'_, '_, '_, T> {
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

        let _ = Writer::new(&participant, &topic).unwrap();
        let _ = Writer::new_with_qos(&participant, &topic, &qos).unwrap();
        let _ = Writer::new(&publisher, &topic).unwrap();
        let _ = Writer::new_with_qos(&publisher, &topic, &qos).unwrap();
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
        let result = Writer::new(&participant, &topic).unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);
        let result = Writer::new_with_qos(&participant, &topic, &qos).unwrap_err();
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
        let writer = Writer::new(&participant, &topic).unwrap();
        writer.write(&Default::default()).unwrap();
    }

    #[test]
    fn test_writer_write_with_timestamp() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain).unwrap();
        let topic = Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();
        let writer = Writer::new(&participant, &topic).unwrap();
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
        let mut writer = Writer::new(&participant, &topic).unwrap();

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
        let mut writer = Writer::new(&participant, &topic).unwrap();

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
