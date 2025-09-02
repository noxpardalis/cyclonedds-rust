//!

use crate::Duration;

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserData {
    ///
    pub value: Vec<u8>,
}

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TopicData {
    ///
    pub value: Vec<u8>,
}

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GroupData {
    ///
    pub value: Vec<u8>,
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

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Deadline {
    ///
    pub period: Duration,
}

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LatencyBudget {
    ///
    pub duration: Duration,
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

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimeBasedFilter {
    ///
    pub minimum_separation: Duration,
}

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Partition {
    ///
    pub partitions: Vec<String>,
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

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransportPriority {
    ///
    pub priority: i32,
}

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Lifespan {
    ///
    pub duration: Duration,
}

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DestinationOrder {
    ///
    ByReceptionTimestamp,
    ///
    BySourceTimestamp,
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

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityFactory {
    ///
    pub autoenable_created_entities: bool,
}

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WriterDataLifecycle {
    ///
    pub autodispose_unregistered_instances: bool,
}

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReaderDataLifecycle {
    ///
    pub autopurge_nowriter_samples_delay: Duration,
    ///
    pub autopurge_disposed_samples_delay: Duration,
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

// ///
// pub struct EntityName {
//     ///
//     pub name: String
// }
