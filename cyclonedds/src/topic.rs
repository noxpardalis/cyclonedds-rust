use crate::internal::ffi;
use crate::internal::sertype::Sertype;
use crate::{Participant, Result};

///
#[derive(Debug)]
pub struct Topic<'domain, 'participant, T>
where
    T: crate::Topicable,
{
    pub(crate) inner: cyclonedds_sys::dds_entity_t,
    phantom_type: std::marker::PhantomData<T>,
    phantom_participant: std::marker::PhantomData<&'participant Participant<'domain>>,
}

///
pub struct TopicBuilder<'domain, 'participant, 'qos, 'name, T>
where
    T: crate::Topicable,
{
    participant: &'participant Participant<'domain>,
    name: &'name str,
    qos: Option<&'qos crate::QoS>,
    phantom: std::marker::PhantomData<T>,
}

impl<'d, 'p, 'q, 'n, T> TopicBuilder<'d, 'p, 'q, 'n, T>
where
    T: crate::Topicable,
{
    pub fn new(participant: &'p Participant<'d>, name: &'n str) -> Self {
        Self {
            participant,
            name,
            qos: None,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn with_qos(mut self, qos: &'q crate::QoS) -> Self {
        self.qos = Some(qos);
        self
    }

    pub fn build(self) -> Result<Topic<'d, 'p, T>> {
        let name =
            std::ffi::CString::new(self.name).map_err(|_| crate::error::Error::BadParameter)?;
        let type_name = std::ffi::CString::new(T::type_name().as_ref())
            .map_err(|_| crate::error::Error::BadParameter)?;

        let mut sertype = std::mem::ManuallyDrop::new(Sertype::<T>::new(&type_name, T::IS_KEYED));

        let inner = ffi::dds_create_topic(
            self.participant.inner,
            &name,
            &mut &mut sertype.inner,
            self.qos.map(|qos| &qos.inner),
            None,
        )
        .inspect_err(|_| {
            ffi::ddsi_sertype_unref(&mut sertype.inner);
        })?;

        Ok(Topic {
            inner,
            phantom_type: std::marker::PhantomData,
            phantom_participant: std::marker::PhantomData,
        })
    }
}

impl<'d, 'p, T> Topic<'d, 'p, T>
where
    T: crate::Topicable,
{
    ///
    pub fn new(participant: &'p Participant<'d>, name: &str) -> Result<Self> {
        Self::builder(participant, name).build()
    }

    ///
    pub fn builder<'q, 'n>(
        participant: &'p Participant<'d>,
        name: &'n str,
    ) -> TopicBuilder<'d, 'p, 'q, 'n, T> {
        TopicBuilder::new(participant, name)
    }

    ///
    pub(crate) const fn from_existing(
        inner: cyclonedds_sys::dds_entity_t,
    ) -> std::mem::ManuallyDrop<Self> {
        std::mem::ManuallyDrop::new(Self {
            inner,
            phantom_type: std::marker::PhantomData,
            phantom_participant: std::marker::PhantomData,
        })
    }
}

impl<T> Drop for Topic<'_, '_, T>
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
    fn test_topic_create() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let qos = crate::QoS::new();
        let topic_name = crate::tests::topic::unique_name();
        let participant = Participant::new(&domain).unwrap();
        let _ = Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();
        let _ = Topic::<crate::tests::topic::Data>::builder(&participant, &topic_name)
            .with_qos(&qos)
            .build()
            .unwrap();
    }

    #[test]
    fn test_topic_create_with_invalid_name() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let qos = crate::QoS::new();
        let topic_name = "\0";
        let participant = Participant::new(&domain).unwrap();

        let result = Topic::<crate::tests::topic::Data>::new(&participant, topic_name).unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);

        let result = Topic::<crate::tests::topic::Data>::builder(&participant, topic_name)
            .with_qos(&qos)
            .build()
            .unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);
    }

    #[test]
    fn test_topic_create_with_invalid_participant() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let qos = crate::QoS::new();
        let topic_name = crate::tests::topic::unique_name();
        let mut participant = Participant::new(&domain).unwrap();
        let participant_id = participant.inner;
        participant.inner = 0;
        let result =
            Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);
        let result = Topic::<crate::tests::topic::Data>::builder(&participant, &topic_name)
            .with_qos(&qos)
            .build()
            .unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);
        participant.inner = participant_id;
    }
}
