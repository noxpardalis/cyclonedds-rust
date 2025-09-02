//!

use crate::Duration;

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserData {
    ///
    pub value: Vec<u8>,
}

impl UserData {
    #[inline]
    pub(crate) fn as_ffi(&self) -> &[u8] {
        &self.value
    }
}

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TopicData {
    ///
    pub value: Vec<u8>,
}

impl TopicData {
    #[inline]
    pub(crate) fn as_ffi(&self) -> &[u8] {
        &self.value
    }
}

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GroupData {
    ///
    pub value: Vec<u8>,
}

impl GroupData {
    #[inline]
    pub(crate) fn as_ffi(&self) -> &[u8] {
        &self.value
    }
}

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Durability {
    ///
    Volatile,
    ///
    TransientLocal,
    ///
    Transient,
    ///
    Persistent,
}

impl Durability {
    #[inline]
    pub(crate) fn as_ffi(&self) -> cyclonedds_sys::dds_durability_kind_t {
        match self {
            Durability::Volatile => cyclonedds_sys::dds_durability_kind_DDS_DURABILITY_VOLATILE,
            Durability::TransientLocal => {
                cyclonedds_sys::dds_durability_kind_DDS_DURABILITY_TRANSIENT_LOCAL
            }
            Durability::Transient => cyclonedds_sys::dds_durability_kind_DDS_DURABILITY_TRANSIENT,
            Durability::Persistent => cyclonedds_sys::dds_durability_kind_DDS_DURABILITY_PERSISTENT,
        }
    }
}

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DurabilityService {
    ///
    pub service_cleanup_delay: Duration,
    ///
    pub history: History,
    ///
    pub max_samples: u32,
    ///
    pub max_instances: u32,
    ///
    pub max_samples_per_instance: u32,
}

impl DurabilityService {
    #[inline]
    pub(crate) fn as_ffi(
        &self,
    ) -> (
        cyclonedds_sys::dds_duration_t,
        cyclonedds_sys::dds_history_kind_t,
        i32,
        i32,
        i32,
        i32,
    ) {
        let (history_kind, history_depth) = self.history.as_ffi();
        let service_cleanup_delay = self.service_cleanup_delay.inner;

        (
            service_cleanup_delay,
            history_kind,
            history_depth,
            self.max_samples as i32,
            self.max_instances as i32,
            self.max_samples_per_instance as i32,
        )
    }
}

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Presentation {
    ///
    Instance {
        ///
        coherent_access: bool,
        ///
        ordered_access: bool,
    },
    ///
    Topic {
        ///
        coherent_access: bool,
        ///
        ordered_access: bool,
    },
    ///
    Group {
        ///
        coherent_access: bool,
        ///
        ordered_access: bool,
    },
}

impl Presentation {
    #[inline]
    pub(crate) fn as_ffi(
        &self,
    ) -> (
        cyclonedds_sys::dds_presentation_access_scope_kind,
        bool,
        bool,
    ) {
        match self {
            Presentation::Instance {
                coherent_access,
                ordered_access,
            } => (
                cyclonedds_sys::dds_presentation_access_scope_kind_DDS_PRESENTATION_INSTANCE,
                *coherent_access,
                *ordered_access,
            ),
            Presentation::Topic {
                coherent_access,
                ordered_access,
            } => (
                cyclonedds_sys::dds_presentation_access_scope_kind_DDS_PRESENTATION_TOPIC,
                *coherent_access,
                *ordered_access,
            ),
            Presentation::Group {
                coherent_access,
                ordered_access,
            } => (
                cyclonedds_sys::dds_presentation_access_scope_kind_DDS_PRESENTATION_GROUP,
                *coherent_access,
                *ordered_access,
            ),
        }
    }
}

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Deadline {
    ///
    pub period: Duration,
}

impl Deadline {
    #[inline]
    pub(crate) fn as_ffi(&self) -> cyclonedds_sys::dds_duration_t {
        self.period.inner
    }
}

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LatencyBudget {
    ///
    pub duration: Duration,
}

impl LatencyBudget {
    #[inline]
    pub(crate) fn as_ffi(&self) -> cyclonedds_sys::dds_duration_t {
        self.duration.inner
    }
}

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Ownership {
    ///
    Shared,
    ///
    Exclusive {
        ///
        strength: i32,
    },
}

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Liveliness {
    ///
    Automatic {
        ///
        lease_duration: Duration,
    },
    ///
    ManualByParticipant {
        ///
        lease_duration: Duration,
    },
    ///
    ManualByTopic {
        ///
        lease_duration: Duration,
    },
}

impl Liveliness {
    #[inline]
    pub(crate) fn as_ffi(
        &self,
    ) -> (
        cyclonedds_sys::dds_liveliness_kind_t,
        cyclonedds_sys::dds_duration_t,
    ) {
        match self {
            Liveliness::Automatic { lease_duration } => (
                cyclonedds_sys::dds_liveliness_kind_DDS_LIVELINESS_AUTOMATIC,
                lease_duration.inner,
            ),
            Liveliness::ManualByParticipant { lease_duration } => (
                cyclonedds_sys::dds_liveliness_kind_DDS_LIVELINESS_MANUAL_BY_PARTICIPANT,
                lease_duration.inner,
            ),
            Liveliness::ManualByTopic { lease_duration } => (
                cyclonedds_sys::dds_liveliness_kind_DDS_LIVELINESS_MANUAL_BY_TOPIC,
                lease_duration.inner,
            ),
        }
    }
}

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimeBasedFilter {
    ///
    pub minimum_separation: Duration,
}

impl TimeBasedFilter {
    #[inline]
    pub(crate) fn as_ffi(&self) -> cyclonedds_sys::dds_duration_t {
        self.minimum_separation.inner
    }
}

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Partition {
    ///
    pub partitions: Vec<String>,
}

impl Partition {
    #[inline]
    pub(crate) fn as_ffi(&self) -> Vec<std::ffi::CString> {
        self.partitions
            .iter()
            .map(|partition| {
                std::ffi::CString::new(partition.as_str()).expect(
                    "TODO should this be moved to the construction of the partition policy?",
                )
            })
            .collect()
    }
}

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Reliability {
    ///
    BestEffort,
    ///
    Reliable {
        ///
        max_blocking_time: Duration,
    },
}

impl Reliability {
    #[inline]
    pub(crate) fn as_ffi(
        &self,
    ) -> (
        cyclonedds_sys::dds_reliability_kind_t,
        cyclonedds_sys::dds_duration_t,
    ) {
        match self {
            Reliability::BestEffort => (
                cyclonedds_sys::dds_reliability_kind_DDS_RELIABILITY_BEST_EFFORT,
                0,
            ),
            Reliability::Reliable { max_blocking_time } => (
                cyclonedds_sys::dds_reliability_kind_DDS_RELIABILITY_RELIABLE,
                max_blocking_time.inner,
            ),
        }
    }
}

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransportPriority {
    ///
    pub priority: i32,
}

impl TransportPriority {
    #[inline]
    pub(crate) fn as_ffi(&self) -> i32 {
        self.priority
    }
}

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Lifespan {
    ///
    pub duration: Duration,
}

impl Lifespan {
    #[inline]
    pub(crate) fn as_ffi(&self) -> cyclonedds_sys::dds_duration_t {
        self.duration.inner
    }
}

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DestinationOrder {
    ///
    ByReceptionTimestamp,
    ///
    BySourceTimestamp,
}

impl DestinationOrder {
    #[inline]
    pub(crate) fn as_ffi(&self) -> cyclonedds_sys::dds_destination_order_kind_t {
        match self {
            DestinationOrder::ByReceptionTimestamp =>
                cyclonedds_sys::dds_destination_order_kind_DDS_DESTINATIONORDER_BY_RECEPTION_TIMESTAMP,
            DestinationOrder::BySourceTimestamp =>
                cyclonedds_sys::dds_destination_order_kind_DDS_DESTINATIONORDER_BY_SOURCE_TIMESTAMP,
        }
    }
}

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum History {
    ///
    KeepAll,
    ///
    KeepLast {
        ///
        depth: i32,
    },
}

impl History {
    #[inline]
    pub(crate) fn as_ffi(&self) -> (cyclonedds_sys::dds_history_kind_t, i32) {
        match self {
            History::KeepAll => (cyclonedds_sys::dds_history_kind_DDS_HISTORY_KEEP_ALL, 0),
            History::KeepLast { depth } => (
                cyclonedds_sys::dds_history_kind_DDS_HISTORY_KEEP_LAST,
                *depth,
            ),
        }
    }
}

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceLimits {
    ///
    pub max_samples: i32,
    ///
    pub max_instances: i32,
    ///
    pub max_samples_per_instance: i32,
}

impl ResourceLimits {
    #[inline]
    pub(crate) fn as_ffi(&self) -> (i32, i32, i32) {
        (
            self.max_samples,
            self.max_instances,
            self.max_samples_per_instance,
        )
    }
}

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityFactory {
    ///
    pub autoenable_created_entities: bool,
}

impl EntityFactory {
    #[inline]
    pub(crate) fn as_ffi(&self) -> bool {
        self.autoenable_created_entities
    }
}

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WriterDataLifecycle {
    ///
    pub autodispose_unregistered_instances: bool,
}

impl WriterDataLifecycle {
    #[inline]
    pub(crate) fn as_ffi(&self) -> bool {
        self.autodispose_unregistered_instances
    }
}

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReaderDataLifecycle {
    ///
    pub autopurge_nowriter_samples_delay: Duration,
    ///
    pub autopurge_disposed_samples_delay: Duration,
}

impl ReaderDataLifecycle {
    #[inline]
    pub(crate) fn as_ffi(
        &self,
    ) -> (
        cyclonedds_sys::dds_duration_t,
        cyclonedds_sys::dds_duration_t,
    ) {
        (
            self.autopurge_nowriter_samples_delay.inner,
            self.autopurge_disposed_samples_delay.inner,
        )
    }
}

// TODO validate the following QoS
// ///
// pub enum IgnoreLocal {
//     ///
//     Nothing,
//     ///
//     Participant,
//     ///
//     Process,
// }

// ///
// pub enum TypeConsistency {
//     ///
//     DisallowTypeCoercion {
//         ///
//         force_type_validation: bool,
//     },
//     ///
//     AllowTypeCoercion {
//         ///
//         ignore_sequence_bounds: bool,
//         ///
//         ignore_string_bounds: bool,
//         ///
//         ignore_member_names: bool,
//         ///
//         prevent_type_widening: bool,
//         ///
//         force_type_validation: bool,
//     },
// }

// ///
// pub struct WriterBatching {
//     ///
//     pub batch_updates: bool,
// }

// ///
// pub struct PsmxInstances {
//     ///
//     pub instances: Vec<String>,
// }

// ///
// pub enum DataRepresentationKind {
//     ///
//     Xcdr1,
//     ///
//     Xml,
//     ///
//     Xcdr2,
// }

// ///
// pub struct DataRepresentation {
//     ///
//     pub representations: std::collections::HashSet<DataRepresentationKind>,
// }

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityName {
    ///
    pub name: String,
}

impl EntityName {
    #[inline]
    pub(crate) fn as_ffi(&self) -> std::ffi::CString {
        std::ffi::CString::new(self.name.as_str()).expect(
            "TODO should this be moved to the construction of the name policy or deferred to the \
             set_qos + construction of the objects with the QoS?",
        )
    }
}
