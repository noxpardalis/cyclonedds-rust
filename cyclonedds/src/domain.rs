//! A Domain defines the boundary of a shared data-space (identified by a 32-bit
//! domain ID). Only entities within the same domain can publish or subscribe to
//! each other’s data.

use crate::internal::ffi;
use crate::{Error, Result};

/// A domain (represented by a domain ID) is a communication space used to
/// isolate publish-subscribe traffic.
#[derive(Debug)]
pub struct Domain {
    pub(crate) id: cyclonedds_sys::dds_domainid_t,
    pub(crate) inner: cyclonedds_sys::dds_entity_t,
}

impl Domain {
    /// Create a new domain with a specified `domain_id`.
    pub fn new(domain_id: u32) -> Result<Self> {
        let inner = ffi::dds_create_domain(domain_id)?;
        Ok(Self {
            id: domain_id,
            inner,
        })
    }

    /// Create a new domain with a specified `domain_id` and a specific XML `config`.
    pub fn new_with_xml_config(domain_id: u32, config: &str) -> Result<Self> {
        let config = std::ffi::CString::new(config).map_err(|_| Error::BadParameter)?;
        let inner = ffi::dds_create_domain_with_config(domain_id, &config)?;
        Ok(Self {
            id: domain_id,
            inner,
        })
    }
}

impl Drop for Domain {
    fn drop(&mut self) {
        if self.inner != 0 {
            let result = ffi::dds_delete(self.inner);
            debug_assert!(result.is_ok())
        }
    }
}

impl Default for Domain {
    fn default() -> Self {
        Self {
            id: cyclonedds_sys::DOMAIN_DEFAULT,
            inner: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_create() {
        let domain_id = crate::tests::domain::unique_id();
        Domain::new(domain_id).unwrap();
    }

    #[test]
    fn test_domain_create_default() {
        let domain = Domain::default();
        assert_eq!(domain.id, cyclonedds_sys::DOMAIN_DEFAULT);
    }

    #[test]
    fn test_domain_create_with_explicit_default_id() {
        let domain_id = cyclonedds_sys::DOMAIN_DEFAULT;
        let domain = Domain::new(domain_id).unwrap_err();
        assert_eq!(domain, Error::BadParameter);
    }

    #[test]
    fn test_domain_create_with_empty_xml_config() {
        let domain_id = crate::tests::domain::unique_id();
        let xml_config = "";
        Domain::new_with_xml_config(domain_id, xml_config).unwrap();
    }

    #[test]
    fn test_domain_create_with_xml_config_with_invalid_string() {
        let domain_id = crate::tests::domain::unique_id();
        let xml_config = "\0";
        let domain = Domain::new_with_xml_config(domain_id, xml_config).unwrap_err();
        assert_eq!(domain, Error::BadParameter);
    }

    #[test]
    fn test_domain_create_with_xml_config_with_explicit_default_id() {
        let domain_id = cyclonedds_sys::DOMAIN_DEFAULT;
        let xml_config = "";
        let domain = Domain::new_with_xml_config(domain_id, xml_config).unwrap_err();
        assert_eq!(domain, Error::BadParameter);
    }

    #[test]
    fn test_domain_create_with_xml_config_with_malformed_xml() {
        let domain_id = crate::tests::domain::unique_id();
        let xml_config = "<>";
        let domain = Domain::new_with_xml_config(domain_id, xml_config).unwrap_err();
        assert_eq!(domain, Error::NonSpecific);
    }

    #[test]
    fn test_domain_create_with_xml_config_with_valid_xml() {
        let domain_id = crate::tests::domain::unique_id();
        let xml_config = "<Domain><General><MaxMessageSize>1400B</MaxMessageSize></General>";
        Domain::new_with_xml_config(domain_id, xml_config).unwrap();
    }
}
