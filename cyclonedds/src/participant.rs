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
}
