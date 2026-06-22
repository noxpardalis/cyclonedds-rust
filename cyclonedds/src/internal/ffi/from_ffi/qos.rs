use super::FromFfi;
use crate::qos::policy;
use crate::{Duration, QoS};

unsafe impl FromFfi for QoS {
    type Source = Option<std::ptr::NonNull<cyclonedds_sys::dds_qos_t>>;

    unsafe fn from_ffi(source: Self::Source) -> Self {
        source
            .map(|source| {
                let source_ref = unsafe { source.as_ref() };
                let source = source.as_ptr();
                let mut qos = QoS::new();

                if let Some(value) =
                    unsafe { octets_from_getter(source, cyclonedds_sys::dds_qget_userdata) }
                {
                    qos = qos.with_user_data(policy::UserData { value });
                }
                if let Some(value) =
                    unsafe { octets_from_getter(source, cyclonedds_sys::dds_qget_topicdata) }
                {
                    qos = qos.with_topic_data(policy::TopicData { value });
                }
                if let Some(value) =
                    unsafe { octets_from_getter(source, cyclonedds_sys::dds_qget_groupdata) }
                {
                    qos = qos.with_group_data(policy::GroupData { value });
                }

                if let Some(durability) = unsafe { extract_durability(source) } {
                    qos = qos.with_durability(durability);
                }
                if let Some(durability_service) = unsafe { extract_durability_service(source) } {
                    qos = qos.with_durability_service(durability_service);
                }
                if let Some(presentation) = unsafe { extract_presentation(source) } {
                    qos = qos.with_presentation(presentation);
                }
                if let Some(deadline) =
                    unsafe { duration_from_getter(source, cyclonedds_sys::dds_qget_deadline) }
                {
                    qos = qos.with_deadline(policy::Deadline { period: deadline });
                }
                if let Some(duration) =
                    unsafe { duration_from_getter(source, cyclonedds_sys::dds_qget_latency_budget) }
                {
                    qos = qos.with_latency_budget(policy::LatencyBudget { duration });
                }
                if let Some(ownership) = unsafe { extract_ownership(source) } {
                    qos = qos.with_ownership(ownership);
                }
                if let Some(liveliness) = unsafe { extract_liveliness(source) } {
                    qos = qos.with_liveliness(liveliness);
                }
                if let Some(minimum_separation) = unsafe {
                    duration_from_getter(source, cyclonedds_sys::dds_qget_time_based_filter)
                } {
                    qos =
                        qos.with_time_based_filter(policy::TimeBasedFilter { minimum_separation });
                }
                if let Some(partition) = unsafe { extract_partition(source) } {
                    qos = qos.with_partition(partition);
                }
                if let Some(reliability) = unsafe { extract_reliability(source) } {
                    qos = qos.with_reliability(reliability);
                }
                if let Some(transport_priority) = unsafe { extract_transport_priority(source) } {
                    qos = qos.with_transport_priority(transport_priority);
                }
                if let Some(duration) =
                    unsafe { duration_from_getter(source, cyclonedds_sys::dds_qget_lifespan) }
                {
                    qos = qos.with_lifespan(policy::Lifespan { duration });
                }
                if let Some(destination_order) = unsafe { extract_destination_order(source) } {
                    qos = qos.with_destination_order(destination_order);
                }
                if let Some(history) = unsafe { extract_history(source) } {
                    qos = qos.with_history(history);
                }
                if let Some(resource_limits) = unsafe { extract_resource_limits(source) } {
                    qos = qos.with_resource_limits(resource_limits);
                }
                if source_ref.present & DDSI_QP_ADLINK_ENTITY_FACTORY != 0 {
                    qos = qos.with_entity_factory(policy::EntityFactory {
                        autoenable_created_entities: source_ref
                            .entity_factory
                            .autoenable_created_entities
                            != 0,
                    });
                }
                if let Some(writer_data_lifecycle) =
                    unsafe { extract_writer_data_lifecycle(source) }
                {
                    qos = qos.with_writer_data_lifecycle(writer_data_lifecycle);
                }
                if let Some(reader_data_lifecycle) =
                    unsafe { extract_reader_data_lifecycle(source) }
                {
                    qos = qos.with_reader_data_lifecycle(reader_data_lifecycle);
                }
                if let Some(entity_name) = unsafe { extract_entity_name(source) } {
                    qos = qos.with_entity_name(entity_name);
                }

                qos
            })
            .unwrap_or_default()
    }
}

type DdsQosOctetsGetter = unsafe extern "C" fn(
    *const cyclonedds_sys::dds_qos_t,
    *mut *mut std::ffi::c_void,
    *mut usize,
) -> bool;
type DdsQosDurationGetter = unsafe extern "C" fn(
    *const cyclonedds_sys::dds_qos_t,
    *mut cyclonedds_sys::dds_duration_t,
) -> bool;

const DDSI_QP_ADLINK_ENTITY_FACTORY: u64 = 1 << 27;

unsafe fn octets_from_getter(
    qos: *const cyclonedds_sys::dds_qos_t,
    getter: DdsQosOctetsGetter,
) -> Option<Vec<u8>> {
    let mut value = std::ptr::null_mut::<std::ffi::c_void>();
    let mut size = 0;

    if unsafe { getter(qos, &raw mut value, &raw mut size) } {
        let result = if value.is_null() || size == 0 {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(value.cast::<u8>(), size) }.to_vec()
        };
        unsafe { dds_free(value) };
        Some(result)
    } else {
        None
    }
}

unsafe fn duration_from_getter(
    qos: *const cyclonedds_sys::dds_qos_t,
    getter: DdsQosDurationGetter,
) -> Option<Duration> {
    let mut duration = 0;
    unsafe { getter(qos, &raw mut duration) }.then(|| Duration::from_nanos(duration))
}

unsafe fn extract_durability(qos: *const cyclonedds_sys::dds_qos_t) -> Option<policy::Durability> {
    let mut kind = 0;
    if !unsafe { cyclonedds_sys::dds_qget_durability(qos, &raw mut kind) } {
        return None;
    }

    match kind {
        cyclonedds_sys::dds_durability_kind_DDS_DURABILITY_VOLATILE => {
            Some(policy::Durability::Volatile)
        }
        cyclonedds_sys::dds_durability_kind_DDS_DURABILITY_TRANSIENT_LOCAL => {
            Some(policy::Durability::TransientLocal)
        }
        cyclonedds_sys::dds_durability_kind_DDS_DURABILITY_TRANSIENT => {
            Some(policy::Durability::Transient)
        }
        cyclonedds_sys::dds_durability_kind_DDS_DURABILITY_PERSISTENT => {
            Some(policy::Durability::Persistent)
        }
        _ => None,
    }
}

unsafe fn extract_durability_service(
    qos: *const cyclonedds_sys::dds_qos_t,
) -> Option<policy::DurabilityService> {
    let mut service_cleanup_delay = 0;
    let mut history_kind = 0;
    let mut history_depth = 0;
    let mut max_samples = 0;
    let mut max_instances = 0;
    let mut max_samples_per_instance = 0;

    if !unsafe {
        cyclonedds_sys::dds_qget_durability_service(
            qos,
            &raw mut service_cleanup_delay,
            &raw mut history_kind,
            &raw mut history_depth,
            &raw mut max_samples,
            &raw mut max_instances,
            &raw mut max_samples_per_instance,
        )
    } {
        return None;
    }

    Some(policy::DurabilityService {
        service_cleanup_delay: Duration::from_nanos(service_cleanup_delay),
        history: history_from_kind(history_kind, history_depth)?,
        resource_limits: resource_limits_from_values(
            max_samples,
            max_instances,
            max_samples_per_instance,
        ),
    })
}

unsafe fn extract_presentation(
    qos: *const cyclonedds_sys::dds_qos_t,
) -> Option<policy::Presentation> {
    let mut access_scope = 0;
    let mut coherent_access = false;
    let mut ordered_access = false;

    if !unsafe {
        cyclonedds_sys::dds_qget_presentation(
            qos,
            &raw mut access_scope,
            &raw mut coherent_access,
            &raw mut ordered_access,
        )
    } {
        return None;
    }

    match access_scope {
        cyclonedds_sys::dds_presentation_access_scope_kind_DDS_PRESENTATION_INSTANCE => {
            Some(policy::Presentation::Instance {
                coherent_access,
                ordered_access,
            })
        }
        cyclonedds_sys::dds_presentation_access_scope_kind_DDS_PRESENTATION_TOPIC => {
            Some(policy::Presentation::Topic {
                coherent_access,
                ordered_access,
            })
        }
        cyclonedds_sys::dds_presentation_access_scope_kind_DDS_PRESENTATION_GROUP => {
            Some(policy::Presentation::Group {
                coherent_access,
                ordered_access,
            })
        }
        _ => None,
    }
}

unsafe fn extract_ownership(qos: *const cyclonedds_sys::dds_qos_t) -> Option<policy::Ownership> {
    let mut kind = 0;
    if !unsafe { cyclonedds_sys::dds_qget_ownership(qos, &raw mut kind) } {
        return None;
    }

    match kind {
        cyclonedds_sys::dds_ownership_kind_DDS_OWNERSHIP_SHARED => Some(policy::Ownership::Shared),
        cyclonedds_sys::dds_ownership_kind_DDS_OWNERSHIP_EXCLUSIVE => {
            let mut strength = 0;
            unsafe {
                cyclonedds_sys::dds_qget_ownership_strength(qos, &raw mut strength);
            }
            Some(policy::Ownership::Exclusive { strength })
        }
        _ => None,
    }
}

unsafe fn extract_liveliness(qos: *const cyclonedds_sys::dds_qos_t) -> Option<policy::Liveliness> {
    let mut kind = 0;
    let mut lease_duration = 0;

    if !unsafe { cyclonedds_sys::dds_qget_liveliness(qos, &raw mut kind, &raw mut lease_duration) }
    {
        return None;
    }

    let lease_duration = Duration::from_nanos(lease_duration);
    match kind {
        cyclonedds_sys::dds_liveliness_kind_DDS_LIVELINESS_AUTOMATIC => {
            Some(policy::Liveliness::Automatic { lease_duration })
        }
        cyclonedds_sys::dds_liveliness_kind_DDS_LIVELINESS_MANUAL_BY_PARTICIPANT => {
            Some(policy::Liveliness::ManualByParticipant { lease_duration })
        }
        cyclonedds_sys::dds_liveliness_kind_DDS_LIVELINESS_MANUAL_BY_TOPIC => {
            Some(policy::Liveliness::ManualByTopic { lease_duration })
        }
        _ => None,
    }
}

unsafe fn extract_partition(qos: *const cyclonedds_sys::dds_qos_t) -> Option<policy::Partition> {
    let mut partition_count = 0;
    let mut partitions = std::ptr::null_mut::<*mut std::ffi::c_char>();

    if !unsafe {
        cyclonedds_sys::dds_qget_partition(qos, &raw mut partition_count, &raw mut partitions)
    } {
        return None;
    }

    let mut result = Vec::with_capacity(partition_count as usize);
    if !partitions.is_null() {
        let partition_slice =
            unsafe { std::slice::from_raw_parts(partitions, partition_count as usize) };
        for &partition in partition_slice
            .iter()
            .filter(|partition| !partition.is_null())
        {
            result.push(
                unsafe { std::ffi::CStr::from_ptr(partition) }
                    .to_string_lossy()
                    .into_owned(),
            );
            unsafe { dds_free(partition) };
        }
        unsafe { dds_free(partitions) };
    }

    Some(policy::Partition { partitions: result })
}

unsafe fn extract_reliability(
    qos: *const cyclonedds_sys::dds_qos_t,
) -> Option<policy::Reliability> {
    let mut kind = 0;
    let mut max_blocking_time = 0;

    if !unsafe {
        cyclonedds_sys::dds_qget_reliability(qos, &raw mut kind, &raw mut max_blocking_time)
    } {
        return None;
    }

    match kind {
        cyclonedds_sys::dds_reliability_kind_DDS_RELIABILITY_BEST_EFFORT => {
            Some(policy::Reliability::BestEffort)
        }
        cyclonedds_sys::dds_reliability_kind_DDS_RELIABILITY_RELIABLE => {
            Some(policy::Reliability::Reliable {
                max_blocking_time: Duration::from_nanos(max_blocking_time),
            })
        }
        _ => None,
    }
}

unsafe fn extract_transport_priority(
    qos: *const cyclonedds_sys::dds_qos_t,
) -> Option<policy::TransportPriority> {
    let mut priority = 0;
    unsafe { cyclonedds_sys::dds_qget_transport_priority(qos, &raw mut priority) }
        .then_some(policy::TransportPriority { priority })
}

unsafe fn extract_destination_order(
    qos: *const cyclonedds_sys::dds_qos_t,
) -> Option<policy::DestinationOrder> {
    let mut kind = 0;
    if !unsafe { cyclonedds_sys::dds_qget_destination_order(qos, &raw mut kind) } {
        return None;
    }

    match kind {
        cyclonedds_sys::dds_destination_order_kind_DDS_DESTINATIONORDER_BY_RECEPTION_TIMESTAMP => {
            Some(policy::DestinationOrder::ByReceptionTimestamp)
        }
        cyclonedds_sys::dds_destination_order_kind_DDS_DESTINATIONORDER_BY_SOURCE_TIMESTAMP => {
            Some(policy::DestinationOrder::BySourceTimestamp)
        }
        _ => None,
    }
}

unsafe fn extract_history(qos: *const cyclonedds_sys::dds_qos_t) -> Option<policy::History> {
    let mut kind = 0;
    let mut depth = 0;

    unsafe { cyclonedds_sys::dds_qget_history(qos, &raw mut kind, &raw mut depth) }
        .then(|| history_from_kind(kind, depth))
        .flatten()
}

unsafe fn extract_resource_limits(
    qos: *const cyclonedds_sys::dds_qos_t,
) -> Option<policy::ResourceLimits> {
    let mut max_samples = 0;
    let mut max_instances = 0;
    let mut max_samples_per_instance = 0;

    unsafe {
        cyclonedds_sys::dds_qget_resource_limits(
            qos,
            &raw mut max_samples,
            &raw mut max_instances,
            &raw mut max_samples_per_instance,
        )
    }
    .then(|| resource_limits_from_values(max_samples, max_instances, max_samples_per_instance))
}

unsafe fn extract_writer_data_lifecycle(
    qos: *const cyclonedds_sys::dds_qos_t,
) -> Option<policy::WriterDataLifecycle> {
    let mut autodispose_unregistered_instances = false;

    unsafe {
        cyclonedds_sys::dds_qget_writer_data_lifecycle(
            qos,
            &raw mut autodispose_unregistered_instances,
        )
    }
    .then_some(policy::WriterDataLifecycle {
        autodispose_unregistered_instances,
    })
}

unsafe fn extract_reader_data_lifecycle(
    qos: *const cyclonedds_sys::dds_qos_t,
) -> Option<policy::ReaderDataLifecycle> {
    let mut autopurge_nowriter_samples_delay = 0;
    let mut autopurge_disposed_samples_delay = 0;

    unsafe {
        cyclonedds_sys::dds_qget_reader_data_lifecycle(
            qos,
            &raw mut autopurge_nowriter_samples_delay,
            &raw mut autopurge_disposed_samples_delay,
        )
    }
    .then_some(policy::ReaderDataLifecycle {
        autopurge_nowriter_samples_delay: Duration::from_nanos(autopurge_nowriter_samples_delay),
        autopurge_disposed_samples_delay: Duration::from_nanos(autopurge_disposed_samples_delay),
    })
}

unsafe fn extract_entity_name(qos: *const cyclonedds_sys::dds_qos_t) -> Option<policy::EntityName> {
    let mut name = std::ptr::null_mut::<std::ffi::c_char>();

    if !unsafe { cyclonedds_sys::dds_qget_entity_name(qos, &raw mut name) } {
        return None;
    }

    let result = if name.is_null() {
        String::new()
    } else {
        unsafe { std::ffi::CStr::from_ptr(name) }
            .to_string_lossy()
            .into_owned()
    };
    unsafe { dds_free(name) };

    Some(policy::EntityName { name: result })
}

const fn history_from_kind(
    kind: cyclonedds_sys::dds_history_kind_t,
    depth: i32,
) -> Option<policy::History> {
    match kind {
        cyclonedds_sys::dds_history_kind_DDS_HISTORY_KEEP_ALL => Some(policy::History::KeepAll),
        cyclonedds_sys::dds_history_kind_DDS_HISTORY_KEEP_LAST => {
            Some(policy::History::KeepLast { depth })
        }
        _ => None,
    }
}

fn resource_limits_from_values(
    max_samples: i32,
    max_instances: i32,
    max_samples_per_instance: i32,
) -> policy::ResourceLimits {
    policy::ResourceLimits {
        max_samples: resource_limit_from_value(max_samples),
        max_instances: resource_limit_from_value(max_instances),
        max_samples_per_instance: resource_limit_from_value(max_samples_per_instance),
    }
}

fn resource_limit_from_value(limit: i32) -> policy::ResourceLimit {
    if limit == cyclonedds_sys::DDS_LENGTH_UNLIMITED {
        policy::ResourceLimit::Unlimited
    } else {
        policy::ResourceLimit::Limited(u32::try_from(limit).unwrap_or_default())
    }
}

unsafe fn dds_free<T>(ptr: *mut T) {
    if !ptr.is_null() {
        unsafe { cyclonedds_sys::dds_free(ptr.cast()) };
    }
}
