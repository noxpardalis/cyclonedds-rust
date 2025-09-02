use crate::Result;
use crate::internal::ffi;

///
#[derive(Debug)]
pub struct Participant<'domain> {
    pub(crate) inner: cyclonedds_sys::dds_entity_t,
    phantom: std::marker::PhantomData<&'domain crate::Domain>,
}

impl<'d> Participant<'d> {
    ///
    pub fn new(domain: &'d crate::Domain, _qos: Option<&crate::QoS>) -> Result<Self> {
        Ok(Self {
            inner: ffi::dds_create_participant(domain.id, None, None)?,
            phantom: Default::default(),
        })
    }
}

impl Drop for Participant<'_> {
    fn drop(&mut self) {
        let result = ffi::dds_delete(self.inner);
        debug_assert!(result.is_ok());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

    #[test]
    fn test_participant_create() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();

        let qos = crate::QoS::new();

        let _ = Participant::new(&domain, None).unwrap();
        let _ = Participant::new(&domain, Some(&qos)).unwrap();
        let _ = Participant::new(&domain, None).unwrap();
        let _ = Participant::new(&domain, Some(&qos)).unwrap();
    }

    #[test]
    fn test_participant_create_in_impossible_domain() {
        let domain = crate::Domain {
            id: u16::MAX as u32,
            inner: 0,
        };

        let result = Participant::new(&domain, None).unwrap_err();
        assert_eq!(result, Error::NonSpecific);
    }
}
