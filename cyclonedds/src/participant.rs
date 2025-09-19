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
    pub fn new(domain: &'d crate::Domain) -> Result<Self> {
        Ok(Self {
            inner: ffi::dds_create_participant(domain.id, None, None)?,
            phantom: Default::default(),
        })
    }

    ///
    pub fn new_with_qos(domain: &'d crate::Domain, qos: &crate::QoS) -> Result<Self> {
        Ok(Self {
            inner: ffi::dds_create_participant(domain.id, Some(&qos.inner), None)?,
            phantom: Default::default(),
        })
    }

    ///
    pub fn set_listener<L>(&mut self, listener: L) -> Result<()>
    where
        L: AsRef<crate::Listener>,
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
        L: AsRef<crate::Listener>,
    {
        self.set_listener(listener).map(|_| self)
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

        let _ = Participant::new(&domain).unwrap();
        let _ = Participant::new_with_qos(&domain, &qos).unwrap();
        let _ = Participant::new(&domain).unwrap();
        let _ = Participant::new_with_qos(&domain, &qos).unwrap();
    }

    #[test]
    fn test_participant_create_in_impossible_domain() {
        let domain = crate::Domain {
            id: u16::MAX as u32,
            inner: 0,
        };

        let result = Participant::new(&domain).unwrap_err();
        assert_eq!(result, Error::NonSpecific);

        let qos = crate::QoS::new();
        let result = Participant::new_with_qos(&domain, &qos).unwrap_err();
        assert_eq!(result, Error::NonSpecific);
    }

    #[test]
    fn test_participant_with_listener() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();

        let listener = crate::Listener::new();

        let _ = Participant::new(&domain)
            .unwrap()
            .with_listener(&listener)
            .unwrap();

        let mut participant = Participant::new(&domain).unwrap();
        participant.set_listener(&listener).unwrap();
        participant.unset_listener().unwrap();
    }

    #[test]
    fn test_participant_with_listener_on_invalid_participant() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();

        let listener = crate::Listener::new();

        let mut participant = Participant::new(&domain).unwrap();
        let participant_id = participant.inner;
        participant.inner = 0;
        let result = participant.set_listener(&listener).unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);
        let result = participant.unset_listener().unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);
        participant.inner = participant_id;
    }
}
