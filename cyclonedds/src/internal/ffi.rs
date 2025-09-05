//! Safe and ergonomic wrappers for the functionality from [`cyclonedds_sys`].
//!
//! This module is exposed to make it easier for applications to poke at the
//! low-level implementation details of Cyclone DDS.

#![allow(unsafe_code)]

pub mod serdata_ops;
pub mod sertype_ops;

use crate::Result;
use crate::error::IntoError;

///
pub fn dds_get_instance_handle(
    entity: cyclonedds_sys::dds_entity_t,
) -> Result<cyclonedds_sys::dds_instance_handle_t> {
    let mut handle = 0;
    unsafe { cyclonedds_sys::dds_get_instance_handle(entity, &mut handle) }.into_error()?;
    Ok(handle)
}

///
pub fn dds_get_status_changes(entity: cyclonedds_sys::dds_entity_t) -> Result<u32> {
    let mut status = 0;
    unsafe { cyclonedds_sys::dds_get_status_changes(entity, &mut status) }.into_error()?;
    Ok(status)
}

///
pub fn dds_read_status(entity: cyclonedds_sys::dds_entity_t, mask: u32) -> Result<u32> {
    let mut status: u32 = 0;
    unsafe { cyclonedds_sys::dds_read_status(entity, &mut status, mask) }.into_error()?;
    Ok(status)
}

///
pub fn dds_take_status(entity: cyclonedds_sys::dds_entity_t, mask: u32) -> Result<u32> {
    let mut status: u32 = 0;
    unsafe { cyclonedds_sys::dds_take_status(entity, &mut status, mask) }.into_error()?;
    Ok(status)
}

///
pub fn dds_get_status_mask(entity: cyclonedds_sys::dds_entity_t) -> Result<u32> {
    let mut mask: u32 = 0;
    unsafe { cyclonedds_sys::dds_get_status_mask(entity, &mut mask) }.into_error()?;
    Ok(mask)
}

///
pub fn dds_set_status_mask(entity: cyclonedds_sys::dds_entity_t, mask: u32) -> Result<()> {
    unsafe { cyclonedds_sys::dds_set_status_mask(entity, mask) }.into_error()?;
    Ok(())
}

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

/// Increment the reference count of a serdata. This is primarily used by the
/// [`Serdata`][`crate::internal::serdata::Serdata`] wrapper.
#[inline]
pub fn ddsi_serdata_ref(serdata: &mut cyclonedds_sys::ddsi_serdata) {
    unsafe { cyclonedds_sys::ddsi_serdata_ref(serdata) };
}

/// Decrement the reference count of a serdata. This is primarily used by the
/// [`Serdata`][`crate::internal::serdata::Serdata`] wrapper.
#[inline]
pub fn ddsi_serdata_unref(serdata: &mut cyclonedds_sys::ddsi_serdata) {
    unsafe { cyclonedds_sys::ddsi_serdata_unref(serdata) };
}

/// Create a new sertype. This is primarily used by the
/// [`Sertype`][`crate::internal::sertype::Sertype`] wrapper.
pub fn ddsi_sertype_new(
    type_name: &std::ffi::CStr,
    sertype_ops: &cyclonedds_sys::ddsi_sertype_ops,
    serdata_ops: &cyclonedds_sys::ddsi_serdata_ops,
    topic_has_key: bool,
) -> cyclonedds_sys::ddsi_sertype {
    let mut sertype = cyclonedds_sys::ddsi_sertype::default();

    unsafe {
        cyclonedds_sys::ddsi_sertype_init(
            &mut sertype,
            type_name.as_ptr(),
            sertype_ops,
            serdata_ops,
            // the interface is based around topic_kind_no_key so invert here.
            !topic_has_key,
        );
    };

    sertype
}

/// Increment the reference count of a sertype. This is primarily used by the
/// [`Sertype`][`crate::internal::sertype::Sertype`] wrapper.
#[inline]
pub fn ddsi_sertype_ref(sertype: &mut cyclonedds_sys::ddsi_sertype) {
    unsafe { cyclonedds_sys::ddsi_sertype_ref(sertype) };
}

/// Decrement the reference count of a sertype. This is primarily used by the
/// [`Sertype`][`crate::internal::sertype::Sertype`] wrapper.
#[inline]
pub fn ddsi_sertype_unref(sertype: &mut cyclonedds_sys::ddsi_sertype) {
    unsafe { cyclonedds_sys::ddsi_sertype_unref(sertype) };
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
/// [`Domain`][`crate::Domain`] wrapper.
pub fn dds_create_domain(
    domain_id: cyclonedds_sys::dds_domainid_t,
) -> Result<cyclonedds_sys::dds_entity_t> {
    Ok(
        unsafe { cyclonedds_sys::dds_create_domain(domain_id, std::ptr::null()) }.into_error()?
            as _,
    )
}

/// Create a domain with a specific XML config. This is primarily used by the
/// [`Domain`][`crate::Domain`] wrapper.
pub fn dds_create_domain_with_config(
    domain_id: cyclonedds_sys::dds_domainid_t,
    config: &std::ffi::CStr,
) -> Result<cyclonedds_sys::dds_entity_t> {
    Ok(unsafe { cyclonedds_sys::dds_create_domain(domain_id, config.as_ptr()) }.into_error()? as _)
}

///
pub fn ddsi_xqos_init(qos: &mut cyclonedds_sys::dds_qos_t) {
    unsafe {
        cyclonedds_sys::ddsi_xqos_init_empty(qos);
    }
}

///
pub fn ddsi_xqos_fini(qos: &mut cyclonedds_sys::dds_qos_t) {
    unsafe {
        cyclonedds_sys::ddsi_xqos_fini(qos);
    }
}

///
pub fn dds_qos_set_user_data(qos: &mut cyclonedds_sys::dds_qos_t, user_data: &[u8]) {
    unsafe {
        cyclonedds_sys::dds_qset_userdata(qos, user_data.as_ptr() as *const _, user_data.len());
    }
}

///
pub fn dds_qos_set_topic_data(qos: &mut cyclonedds_sys::dds_qos_t, topic_data: &[u8]) {
    unsafe {
        cyclonedds_sys::dds_qset_userdata(qos, topic_data.as_ptr() as *const _, topic_data.len());
    }
}

///
pub fn dds_qos_set_group_data(qos: &mut cyclonedds_sys::dds_qos_t, group_data: &[u8]) {
    unsafe {
        cyclonedds_sys::dds_qset_userdata(qos, group_data.as_ptr() as *const _, group_data.len());
    }
}

///
pub fn dds_qos_set_durability(
    qos: &mut cyclonedds_sys::dds_qos_t,
    kind: cyclonedds_sys::dds_durability_kind_t,
) {
    unsafe { cyclonedds_sys::dds_qset_durability(qos, kind) }
}

///
pub fn dds_qos_set_durability_service(
    qos: &mut cyclonedds_sys::dds_qos_t,
    service_cleanup_delay: cyclonedds_sys::dds_duration_t,
    history_kind: cyclonedds_sys::dds_history_kind_t,
    history_depth: i32,
    max_samples: i32,
    max_instances: i32,
    max_samples_per_instance: i32,
) {
    unsafe {
        cyclonedds_sys::dds_qset_durability_service(
            qos,
            service_cleanup_delay,
            history_kind,
            history_depth,
            max_samples,
            max_instances,
            max_samples_per_instance,
        )
    }
}

///
pub fn dds_qos_set_presentation(
    qos: &mut cyclonedds_sys::dds_qos_t,
    access_scope: cyclonedds_sys::dds_presentation_access_scope_kind,
    coherent_access: bool,
    ordered_access: bool,
) {
    unsafe {
        cyclonedds_sys::dds_qset_presentation(qos, access_scope, coherent_access, ordered_access)
    }
}

///
pub fn dds_qos_set_deadline(
    qos: &mut cyclonedds_sys::dds_qos_t,
    deadline: cyclonedds_sys::dds_duration_t,
) {
    unsafe { cyclonedds_sys::dds_qset_deadline(qos, deadline) }
}

///
pub fn dds_qos_set_latency_budget(
    qos: &mut cyclonedds_sys::dds_qos_t,
    duration: cyclonedds_sys::dds_duration_t,
) {
    unsafe { cyclonedds_sys::dds_qset_latency_budget(qos, duration) }
}

///
pub fn dds_qos_set_ownership(
    qos: &mut cyclonedds_sys::dds_qos_t,
    kind: cyclonedds_sys::dds_ownership_kind_t,
) {
    unsafe { cyclonedds_sys::dds_qset_ownership(qos, kind) }
}

///
pub fn dds_qos_set_ownership_strength(qos: &mut cyclonedds_sys::dds_qos_t, value: i32) {
    unsafe { cyclonedds_sys::dds_qset_ownership_strength(qos, value) }
}

///
pub fn dds_qos_set_liveliness(
    qos: &mut cyclonedds_sys::dds_qos_t,
    kind: cyclonedds_sys::dds_liveliness_kind_t,
    lease_duration: cyclonedds_sys::dds_duration_t,
) {
    unsafe { cyclonedds_sys::dds_qset_liveliness(qos, kind, lease_duration) }
}

///
pub fn dds_qos_set_time_based_filter(
    qos: &mut cyclonedds_sys::dds_qos_t,
    minimum_separation: cyclonedds_sys::dds_duration_t,
) {
    unsafe { cyclonedds_sys::dds_qset_time_based_filter(qos, minimum_separation) }
}

///
pub fn dds_qos_set_partition(
    qos: &mut cyclonedds_sys::dds_qos_t,
    partitions: Vec<std::ffi::CString>,
) {
    let n = partitions.len() as u32;
    let mut ps: Vec<_> = partitions.iter().map(|str| str.as_ptr()).collect();
    unsafe { cyclonedds_sys::dds_qset_partition(qos, n, ps.as_mut_ptr()) }
}

///
pub fn dds_qos_set_reliability(
    qos: &mut cyclonedds_sys::dds_qos_t,
    kind: cyclonedds_sys::dds_reliability_kind_t,
    max_blocking_time: cyclonedds_sys::dds_duration_t,
) {
    unsafe { cyclonedds_sys::dds_qset_reliability(qos, kind, max_blocking_time) }
}

///
pub fn dds_qos_set_transport_priority(qos: &mut cyclonedds_sys::dds_qos_t, value: i32) {
    unsafe { cyclonedds_sys::dds_qset_transport_priority(qos, value) }
}

///
pub fn dds_qos_set_lifespan(
    qos: &mut cyclonedds_sys::dds_qos_t,
    lifespan: cyclonedds_sys::dds_duration_t,
) {
    unsafe { cyclonedds_sys::dds_qset_lifespan(qos, lifespan) }
}

///
pub fn dds_qos_set_destination_order(
    qos: &mut cyclonedds_sys::dds_qos_t,
    kind: cyclonedds_sys::dds_destination_order_kind_t,
) {
    unsafe { cyclonedds_sys::dds_qset_destination_order(qos, kind) }
}

///
pub fn dds_qos_set_history(
    qos: &mut cyclonedds_sys::dds_qos_t,
    kind: cyclonedds_sys::dds_history_kind_t,
    depth: i32,
) {
    unsafe { cyclonedds_sys::dds_qset_history(qos, kind, depth) }
}

///
pub fn dds_qos_set_resource_limits(
    qos: &mut cyclonedds_sys::dds_qos_t,
    max_samples: i32,
    max_instances: i32,
    max_samples_per_instance: i32,
) {
    unsafe {
        cyclonedds_sys::dds_qset_resource_limits(
            qos,
            max_samples,
            max_instances,
            max_samples_per_instance,
        )
    }
}

///
pub fn dds_qos_set_entity_factory(
    qos: &mut cyclonedds_sys::dds_qos_t,
    autoenable_created_entities: bool,
) {
    // FIXME? there's no associated dds_qset_?
    qos.entity_factory.autoenable_created_entities =
        if autoenable_created_entities { 1 } else { 0 };
}

///
pub fn dds_qos_set_writer_data_lifecycle(qos: &mut cyclonedds_sys::dds_qos_t, autodispose: bool) {
    unsafe { cyclonedds_sys::dds_qset_writer_data_lifecycle(qos, autodispose) }
}

///
pub fn dds_qos_set_reader_data_lifecycle(
    qos: &mut cyclonedds_sys::dds_qos_t,
    autopurge_nowriter_samples_delay: cyclonedds_sys::dds_duration_t,
    autopurge_disposed_samples_delay: cyclonedds_sys::dds_duration_t,
) {
    unsafe {
        cyclonedds_sys::dds_qset_reader_data_lifecycle(
            qos,
            autopurge_nowriter_samples_delay,
            autopurge_disposed_samples_delay,
        )
    }
}

///
pub fn dds_qos_set_entity_name(qos: &mut cyclonedds_sys::dds_qos_t, name: std::ffi::CString) {
    unsafe { cyclonedds_sys::dds_qset_entity_name(qos, name.as_ptr()) }
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

/// Create a topic under a participant. This is primarily used by the
/// [`Topic`][`crate::Topic`] wrapper.
pub fn dds_create_topic(
    participant: cyclonedds_sys::dds_entity_t,
    name: &std::ffi::CStr,
    sertype: &mut &mut cyclonedds_sys::ddsi_sertype,
    qos: Option<&cyclonedds_sys::dds_qos_t>,
    listener: Option<&cyclonedds_sys::dds_listener_t>,
) -> Result<cyclonedds_sys::dds_entity_t> {
    let sedp_plist = std::ptr::null();

    unsafe {
        cyclonedds_sys::dds_create_topic_sertype(
            participant,
            name.as_ptr(),
            sertype as *mut &mut _ as *mut *mut _,
            qos.map(|qos| qos as *const _).unwrap_or(std::ptr::null()),
            listener
                .map(|listener| listener as *const _)
                .unwrap_or(std::ptr::null()),
            sedp_plist,
        )
    }
    .into_error()
}

/// Create a publisher under a participant. This is primarily used by the
/// [`Publisher`][`crate::Publisher`] wrapper.
pub fn dds_create_publisher(
    participant: cyclonedds_sys::dds_entity_t,
    qos: Option<&cyclonedds_sys::dds_qos_t>,
    listener: Option<&cyclonedds_sys::dds_listener_t>,
) -> Result<cyclonedds_sys::dds_entity_t> {
    unsafe {
        cyclonedds_sys::dds_create_publisher(
            participant,
            qos.map(|qos| qos as *const _).unwrap_or(std::ptr::null()),
            listener
                .map(|listener| listener as *const _)
                .unwrap_or(std::ptr::null()),
        )
    }
    .into_error()
}

///
pub fn dds_suspend(publisher: cyclonedds_sys::dds_entity_t) -> Result<()> {
    unsafe { cyclonedds_sys::dds_suspend(publisher) }.into_error()?;
    Ok(())
}

///
pub fn dds_resume(publisher: cyclonedds_sys::dds_entity_t) -> Result<()> {
    unsafe { cyclonedds_sys::dds_resume(publisher) }.into_error()?;
    Ok(())
}

///
pub fn dds_wait_for_acks(
    publisher_or_writer: cyclonedds_sys::dds_entity_t,
    timeout: cyclonedds_sys::dds_duration_t,
) -> Result<()> {
    unsafe { cyclonedds_sys::dds_wait_for_acks(publisher_or_writer, timeout) }.into_error()?;
    Ok(())
}

/// Create a subscriber under a participant. This is primarily used by the
/// [`Subscriber`][`crate::Subscriber`] wrapper.
pub fn dds_create_subscriber(
    participant: cyclonedds_sys::dds_entity_t,
    qos: Option<&cyclonedds_sys::dds_qos_t>,
    listener: Option<&cyclonedds_sys::dds_listener_t>,
) -> Result<cyclonedds_sys::dds_entity_t> {
    unsafe {
        cyclonedds_sys::dds_create_subscriber(
            participant,
            qos.map(|qos| qos as *const _).unwrap_or(std::ptr::null()),
            listener
                .map(|listener| listener as *const _)
                .unwrap_or(std::ptr::null()),
        )
    }
    .into_error()
}

///
pub fn dds_notify_readers(subscriber: cyclonedds_sys::dds_entity_t) -> Result<()> {
    unsafe { cyclonedds_sys::dds_notify_readers(subscriber) }.into_error()?;
    Ok(())
}

#[cfg(test)]
mod tests;
