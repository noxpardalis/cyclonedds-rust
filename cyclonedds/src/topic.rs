use crate::Participant;
use crate::QoS;
use crate::Result;
use crate::internal::ffi;
use crate::internal::sertype::Sertype;

///
#[derive(Debug)]
pub struct Topic<'domain, 'participant, T> {
    pub(crate) inner: cyclonedds_sys::dds_entity_t,
    _sertype: Box<Sertype<T>>,
    phantom_participant: std::marker::PhantomData<&'participant Participant<'domain>>,
}

impl<'d, 'p, T> Topic<'d, 'p, T>
where
    T: serde::ser::Serialize + serde::de::DeserializeOwned + std::clone::Clone + Default,
{
    ///
    pub fn new(participant: &'p Participant<'d>, name: &str, qos: Option<&QoS>) -> Result<Self> {
        let name = std::ffi::CString::new(name).map_err(|_| crate::error::Error::BadParameter)?;
        let mut sertype = Sertype::<T>::new(&name, false);
        let inner = ffi::dds_create_topic(
            participant.inner,
            &name,
            &mut &mut sertype.inner,
            qos.map(|qos| &qos.inner),
            None,
        )?;

        Ok(Self {
            inner,
            _sertype: sertype,
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
        let topic_name = crate::tests::topic::unique_name();
        let participant = Participant::new(&domain, None).unwrap();
        let _ = Topic::<crate::tests::topic::Data>::new(&participant, &topic_name, None).unwrap();
    }

    #[test]
    fn test_topic_create_with_invalid_name() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = "\0";
        let participant = Participant::new(&domain, None).unwrap();
        let result =
            Topic::<crate::tests::topic::Data>::new(&participant, topic_name, None).unwrap_err();

        assert_eq!(result, crate::Error::BadParameter);
    }

    #[test]
    fn test_topic_create_with_invalid_participant() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let mut participant = Participant::new(&domain, None).unwrap();
        let participant_id = participant.inner;
        participant.inner = 0;
        let result =
            Topic::<crate::tests::topic::Data>::new(&participant, &topic_name, None).unwrap_err();
        participant.inner = participant_id;

        assert_eq!(result, crate::Error::BadParameter);
    }
}
