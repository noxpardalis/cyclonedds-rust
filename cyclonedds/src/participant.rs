use crate::Result;
use crate::internal::ffi;

///
#[derive(Debug)]
pub struct Participant<'domain> {
    pub(crate) inner: cyclonedds_sys::dds_entity_t,
    phantom: std::marker::PhantomData<&'domain crate::Domain>,
}

pub struct ParticipantBuilder<'domain, 'qos> {
    domain: &'domain crate::Domain,
    qos: Option<&'qos crate::QoS>,
}

impl<'d, 'q> ParticipantBuilder<'d, 'q> {
    pub fn new(domain: &'d crate::Domain) -> Self {
        Self { domain, qos: None }
    }

    pub fn with_qos(mut self, qos: &'q crate::QoS) -> Self {
        self.qos = Some(qos);
        self
    }

    pub fn build(self) -> Result<Participant<'d>> {
        Ok(Participant {
            inner: ffi::dds_create_participant(
                self.domain.id,
                self.qos.map(|qos| &qos.inner),
                None,
            )?,
            phantom: std::marker::PhantomData,
        })
    }
}

impl<'d> Participant<'d> {
    ///
    pub fn new(domain: &'d crate::Domain) -> Result<Self> {
        Self::builder(domain).build()
    }

    ///
    pub fn builder<'q>(domain: &'d crate::Domain) -> ParticipantBuilder<'d, 'q> {
        ParticipantBuilder::new(domain)
    }

    ///
    pub fn set_listener<L>(&mut self, listener: L) -> Result<()>
    where
        L: AsRef<crate::Listener>,
    {
        listener
            .as_ref()
            .as_ffi()
            .and_then(|listener| ffi::dds_set_listener(self.inner, Some(listener.inner)))
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
        let _ = Participant::builder(&domain)
            .with_qos(&qos)
            .build()
            .unwrap();
        let _ = Participant::new(&domain).unwrap();
        let _ = Participant::builder(&domain)
            .with_qos(&qos)
            .build()
            .unwrap();
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
        let result = Participant::builder(&domain)
            .with_qos(&qos)
            .build()
            .unwrap_err();
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
