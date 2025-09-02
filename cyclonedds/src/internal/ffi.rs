//! Safe and ergonomic wrappers for the functionality from [`cyclonedds_sys`].
//!
//! This module is exposed to make it easier for applications to poke at the
//! low-level implementation details of Cyclone DDS.

#![allow(unsafe_code)]

pub mod serdata_ops;
pub mod sertype_ops;

use crate::Result;
use crate::error::IntoError;

/// Create a new serdata. This is primarily used by the
/// [`Serdata`][`crate::internal::serdata::Serdata`] wrapper.
pub fn ddsi_serdata_new(
    sertype: &cyclonedds_sys::ddsi_sertype,
    kind: cyclonedds_sys::ddsi_serdata_kind,
) -> cyclonedds_sys::ddsi_serdata {
    let mut serdata = cyclonedds_sys::ddsi_serdata::default();

    unsafe { cyclonedds_sys::ddsi_serdata_init(&mut serdata, sertype, kind) }
    serdata
}

/// Create a new sertype. This is primarily used by the
/// [`Sertype`][`crate::internal::sertype::Sertype`] wrapper.
pub fn ddsi_sertype_new(
    type_name: &std::ffi::CStr,
    sertype_ops: &cyclonedds_sys::ddsi_sertype_ops,
    serdata_ops: &cyclonedds_sys::ddsi_serdata_ops,
    topic_kind_no_key: bool,
) -> cyclonedds_sys::ddsi_sertype {
    let mut sertype = cyclonedds_sys::ddsi_sertype::default();

    unsafe {
        cyclonedds_sys::ddsi_sertype_init(
            &mut sertype,
            type_name.as_ptr(),
            sertype_ops,
            serdata_ops,
            topic_kind_no_key,
        );
    };

    sertype
}

/// Finalize a sertype. This is primarily used by the
/// [`Sertype`][`crate::internal::sertype::Sertype`] wrapper.
pub fn ddsi_sertype_fini(sertype: &mut cyclonedds_sys::ddsi_sertype) {
    unsafe {
        cyclonedds_sys::ddsi_sertype_fini(sertype);
    }
}

/// Delete an entity.
pub fn dds_delete(entity: cyclonedds_sys::dds_entity_t) -> Result<()> {
    unsafe { cyclonedds_sys::dds_delete(entity) }.into_error()?;
    Ok(())
}

/// Create a domain. This is primarily used by the
/// [`Domain`][`crate::core::Domain`] wrapper.
pub fn dds_create_domain(
    domain_id: cyclonedds_sys::dds_domainid_t,
) -> Result<cyclonedds_sys::dds_entity_t> {
    Ok(
        unsafe { cyclonedds_sys::dds_create_domain(domain_id, std::ptr::null()) }.into_error()?
            as _,
    )
}

/// Create a domain with a specific XML config. This is primarily used by the
/// [`Domain`][`crate::core::Domain`] wrapper.
pub fn dds_create_domain_with_config(
    domain_id: cyclonedds_sys::dds_domainid_t,
    config: &std::ffi::CStr,
) -> Result<cyclonedds_sys::dds_entity_t> {
    Ok(unsafe { cyclonedds_sys::dds_create_domain(domain_id, config.as_ptr()) }.into_error()? as _)
}

/// Create a participant within a domain. This is primarily used by the
/// [`Participant`][`crate::Participant`] wrapper.
pub fn dds_create_participant(
    domain: cyclonedds_sys::dds_domainid_t,
    qos: Option<&cyclonedds_sys::dds_qos_t>,
    listener: Option<&cyclonedds_sys::dds_listener_t>,
) -> Result<cyclonedds_sys::dds_entity_t> {
    unsafe {
        cyclonedds_sys::dds_create_participant(
            domain,
            qos.map(|qos| qos as *const _).unwrap_or(std::ptr::null()),
            listener
                .map(|listener| listener as *const _)
                .unwrap_or(std::ptr::null()),
        )
    }
    .into_error()
}

#[cfg(test)]
mod tests;
