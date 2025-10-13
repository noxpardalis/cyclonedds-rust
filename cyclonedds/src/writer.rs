use crate::Result;
use crate::{ParticipantOrPublisher, Topic};

use crate::internal::ffi;

///
#[derive(Debug)]
pub struct Writer<'domain, 'participant, 'topic, T>
where
    T: crate::sample::Keyed,
{
    pub(crate) inner: cyclonedds_sys::dds_entity_t,
    phantom_topic: std::marker::PhantomData<&'topic Topic<'domain, 'participant, T>>,
}

impl<'d, 'p, 't, T> Writer<'d, 'p, 't, T>
where
    T: crate::sample::Keyed,
{
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
            phantom_topic: std::marker::PhantomData,
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
            phantom_topic: std::marker::PhantomData,
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
    pub(crate) const fn from_existing(
        inner: cyclonedds_sys::dds_entity_t,
    ) -> std::mem::ManuallyDrop<Self> {
        std::mem::ManuallyDrop::new(Self {
            inner,
            phantom_topic: std::marker::PhantomData,
        })
    }

    pub fn set_listener<L>(&mut self, listener: L) -> Result<()>
    where
        T: serde::ser::Serialize
            + serde::de::DeserializeOwned
            + std::clone::Clone
            + std::default::Default
            + std::fmt::Debug,
        L: AsRef<crate::WriterListener<T>>,
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
        T: serde::ser::Serialize
            + serde::de::DeserializeOwned
            + std::clone::Clone
            + std::default::Default
            + std::fmt::Debug,
        L: AsRef<crate::WriterListener<T>>,
    {
        self.set_listener(listener).map(|_| self)
    }
}

impl<T> Drop for Writer<'_, '_, '_, T>
where
    T: crate::sample::Keyed,
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

    #[test]
    fn test_writer_with_listener() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain).unwrap();
        let topic = Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();

        let listener = crate::WriterListener::new()
            .with_liveliness_lost(|_, _| ())
            .with_offered_deadline_missed(|_, _| ())
            .with_offered_incompatible_qos(|_, _| ())
            .with_publication_matched(|_, _| ());

        let _ = Writer::new(&participant, &topic)
            .unwrap()
            .with_listener(&listener)
            .unwrap();

        let mut writer = Writer::new(&participant, &topic).unwrap();
        writer.set_listener(&listener).unwrap();
        writer.unset_listener().unwrap();
    }

    #[test]
    fn test_writer_with_listener_on_invalid_writer() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain).unwrap();
        let topic = Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();

        let listener = crate::WriterListener::new();

        let mut writer = Writer::new(&participant, &topic).unwrap();
        let writer_id = writer.inner;
        writer.inner = 0;
        let result = writer.set_listener(&listener).unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);
        let result = writer.unset_listener().unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);
        writer.inner = writer_id;
    }

    #[test]
    fn test_writer_create_from_existing() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain).unwrap();
        let topic = Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();

        let writer_01 = Writer::new(&participant, &topic).unwrap();
        let writer_02 = Writer::<crate::tests::topic::Data>::from_existing(writer_01.inner);
        assert_eq!(writer_01.inner, writer_02.inner);
    }
}
