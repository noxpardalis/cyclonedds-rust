use crate::Participant;
use crate::QoS;
use crate::Result;
use crate::internal::ffi;
use crate::internal::sertype::Sertype;

///
#[derive(Debug)]
pub struct Topic<'domain, 'participant, T> {
    pub(crate) inner: cyclonedds_sys::dds_entity_t,
    _sertype: Option<Box<Sertype<T>>>,
    phantom_participant: std::marker::PhantomData<&'participant Participant<'domain>>,
}

impl<'d, 'p, T> Topic<'d, 'p, T>
where
    T: serde::ser::Serialize + serde::de::DeserializeOwned + std::clone::Clone + Default,
{
    ///
    pub fn new(participant: &'p Participant<'d>, name: &str) -> Result<Self> {
        let name = std::ffi::CString::new(name).map_err(|_| crate::error::Error::BadParameter)?;
        let mut sertype = Sertype::<T>::new(&name, false);
        let inner = ffi::dds_create_topic(
            participant.inner,
            &name,
            &mut &mut sertype.inner,
            None,
            None,
        )?;

        Ok(Self {
            inner,
            _sertype: Some(sertype),
            phantom_participant: Default::default(),
        })
    }

    ///
    pub fn new_with_qos(participant: &'p Participant<'d>, name: &str, qos: &QoS) -> Result<Self> {
        let name = std::ffi::CString::new(name).map_err(|_| crate::error::Error::BadParameter)?;
        let mut sertype = Sertype::<T>::new(&name, false);
        let inner = ffi::dds_create_topic(
            participant.inner,
            &name,
            &mut &mut sertype.inner,
            Some(&qos.inner),
            None,
        )?;

        Ok(Self {
            inner,
            _sertype: Some(sertype),
            phantom_participant: Default::default(),
        })
    }

    ///
    pub(crate) fn from_existing(
        inner: cyclonedds_sys::dds_entity_t,
    ) -> std::mem::ManuallyDrop<Self> {
        std::mem::ManuallyDrop::new(Self {
            inner,
            _sertype: None,
            phantom_participant: Default::default(),
        })
    }
}

impl<T> Drop for Topic<'_, '_, T> {
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
        let _ = Topic::<crate::tests::topic::Data>::new_with_qos(&participant, &topic_name, &qos)
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

        let result =
            Topic::<crate::tests::topic::Data>::new_with_qos(&participant, topic_name, &qos)
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
        let result =
            Topic::<crate::tests::topic::Data>::new_with_qos(&participant, &topic_name, &qos)
                .unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);
        participant.inner = participant_id;
    }
}
