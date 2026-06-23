//! Quality of Service ([`QoS`]) policies for DDS entities.
//!
//! [`QoS`] is built through a chainable builder and applied to entities via
//! their respective builders. Policies are defined in the [`policy`] submodule.
//!
//! # Examples
//!
//! ```
//! use cyclonedds::qos::policy;
//! use cyclonedds::{Duration, QoS};
//!
//! let qos = QoS::new()
//!     .with_durability(policy::Durability::TransientLocal)
//!     .with_history(policy::History::KeepAll)
//!     .with_reliability(policy::Reliability::Reliable {
//!         max_blocking_time: Duration::INFINITE,
//!     });
//! ```

pub mod policy;

use crate::internal::ffi;
use crate::internal::traits::AsFfi;

/// A set of Quality of Service [`policies`](policy) applied to a DDS entity.
///
/// Constructed via [`QoS::new`] and configured through chainable `with_*`
/// methods. Unset policies inherit the defaults for the entity type they are
/// applied to.
#[derive(Debug, Clone, Default)]
pub struct QoS {
    user_data: Option<policy::UserData>,
    topic_data: Option<policy::TopicData>,
    group_data: Option<policy::GroupData>,
    durability: Option<policy::Durability>,
    durability_service: Option<policy::DurabilityService>,
    presentation: Option<policy::Presentation>,
    deadline: Option<policy::Deadline>,
    latency_budget: Option<policy::LatencyBudget>,
    ownership: Option<policy::Ownership>,
    liveliness: Option<policy::Liveliness>,
    time_based_filter: Option<policy::TimeBasedFilter>,
    partition: Option<policy::Partition>,
    reliability: Option<policy::Reliability>,
    transport_priority: Option<policy::TransportPriority>,
    lifespan: Option<policy::Lifespan>,
    destination_order: Option<policy::DestinationOrder>,
    history: Option<policy::History>,
    resource_limits: Option<policy::ResourceLimits>,
    entity_factory: Option<policy::EntityFactory>,
    writer_data_lifecycle: Option<policy::WriterDataLifecycle>,
    reader_data_lifecycle: Option<policy::ReaderDataLifecycle>,
    entity_name: Option<policy::EntityName>,
}

impl AsFfi for QoS {
    type Target<'a>
        = cyclonedds_sys::dds_qos_t
    where
        Self: 'a;

    fn as_ffi(&self) -> Self::Target<'_> {
        let mut target = cyclonedds_sys::dds_qos_t {
            present: 0,
            aliased: 0,
            ..Default::default()
        };

        self.apply_user_data_qos(&mut target);
        self.apply_topic_data_qos(&mut target);
        self.apply_group_data_qos(&mut target);
        self.apply_durability_qos(&mut target);
        self.apply_durability_service_qos(&mut target);
        self.apply_presentation_qos(&mut target);
        self.apply_deadline_qos(&mut target);
        self.apply_latency_budget_qos(&mut target);
        self.apply_ownership_qos(&mut target);
        self.apply_liveliness_qos(&mut target);
        self.apply_time_based_filter_qos(&mut target);
        self.apply_partition_qos(&mut target);
        self.apply_reliability_qos(&mut target);
        self.apply_transport_priority_qos(&mut target);
        self.apply_lifespan_qos(&mut target);
        self.apply_destination_order_qos(&mut target);
        self.apply_history_qos(&mut target);
        self.apply_resource_limits_qos(&mut target);
        self.apply_entity_factory_qos(&mut target);
        self.apply_writer_data_lifecycle_qos(&mut target);
        self.apply_reader_data_lifecycle_qos(&mut target);
        self.apply_entity_name_qos(&mut target);

        target
    }
}

impl QoS {
    fn apply_user_data_qos(&self, target: &mut cyclonedds_sys::dds_qos_t) {
        if let Some(user_data) = &self.user_data {
            ffi::dds_qos_set_user_data(target, user_data.as_ffi());
        }
    }

    fn apply_topic_data_qos(&self, target: &mut cyclonedds_sys::dds_qos_t) {
        if let Some(topic_data) = &self.topic_data {
            ffi::dds_qos_set_topic_data(target, topic_data.as_ffi());
        }
    }

    fn apply_group_data_qos(&self, target: &mut cyclonedds_sys::dds_qos_t) {
        if let Some(group_data) = &self.group_data {
            ffi::dds_qos_set_group_data(target, group_data.as_ffi());
        }
    }

    fn apply_durability_qos(&self, target: &mut cyclonedds_sys::dds_qos_t) {
        if let Some(durability) = &self.durability {
            ffi::dds_qos_set_durability(target, durability.as_ffi());
        }
    }

    fn apply_durability_service_qos(&self, target: &mut cyclonedds_sys::dds_qos_t) {
        if let Some(durability_service) = &self.durability_service {
            let (
                service_cleanup_delay,
                history_kind,
                history_depth,
                max_samples,
                max_instances,
                max_samples_per_instance,
            ) = durability_service.as_ffi();
            ffi::dds_qos_set_durability_service(
                target,
                service_cleanup_delay,
                history_kind,
                history_depth,
                max_samples,
                max_instances,
                max_samples_per_instance,
            );
        }
    }

    fn apply_presentation_qos(&self, target: &mut cyclonedds_sys::dds_qos_t) {
        if let Some(presentation) = &self.presentation {
            let (access_scope, coherent_access, ordered_access) = presentation.as_ffi();
            ffi::dds_qos_set_presentation(target, access_scope, coherent_access, ordered_access);
        }
    }

    fn apply_deadline_qos(&self, target: &mut cyclonedds_sys::dds_qos_t) {
        if let Some(deadline) = &self.deadline {
            ffi::dds_qos_set_deadline(target, deadline.as_ffi());
        }
    }

    fn apply_latency_budget_qos(&self, target: &mut cyclonedds_sys::dds_qos_t) {
        if let Some(latency_budget) = &self.latency_budget {
            ffi::dds_qos_set_latency_budget(target, latency_budget.as_ffi());
        }
    }

    fn apply_ownership_qos(&self, target: &mut cyclonedds_sys::dds_qos_t) {
        if let Some(ownership) = &self.ownership {
            let (kind, strength) = ownership.as_ffi();
            ffi::dds_qos_set_ownership(target, kind);
            if let Some(strength) = strength {
                ffi::dds_qos_set_ownership_strength(target, strength);
            }
        }
    }

    fn apply_liveliness_qos(&self, target: &mut cyclonedds_sys::dds_qos_t) {
        if let Some(liveliness) = &self.liveliness {
            let (kind, lease_duration) = liveliness.as_ffi();
            ffi::dds_qos_set_liveliness(target, kind, lease_duration);
        }
    }

    fn apply_time_based_filter_qos(&self, target: &mut cyclonedds_sys::dds_qos_t) {
        if let Some(time_based_filter) = &self.time_based_filter {
            ffi::dds_qos_set_time_based_filter(target, time_based_filter.as_ffi());
        }
    }

    fn apply_partition_qos(&self, target: &mut cyclonedds_sys::dds_qos_t) {
        if let Some(partition) = &self.partition {
            let partitions = partition.as_ffi();
            ffi::dds_qos_set_partition(target, &partitions);
        }
    }

    fn apply_reliability_qos(&self, target: &mut cyclonedds_sys::dds_qos_t) {
        if let Some(reliability) = &self.reliability {
            let (kind, max_blocking_time) = reliability.as_ffi();
            ffi::dds_qos_set_reliability(target, kind, max_blocking_time);
        }
    }

    fn apply_transport_priority_qos(&self, target: &mut cyclonedds_sys::dds_qos_t) {
        if let Some(transport_priority) = &self.transport_priority {
            ffi::dds_qos_set_transport_priority(target, transport_priority.as_ffi());
        }
    }

    fn apply_lifespan_qos(&self, target: &mut cyclonedds_sys::dds_qos_t) {
        if let Some(lifespan) = &self.lifespan {
            ffi::dds_qos_set_lifespan(target, lifespan.as_ffi());
        }
    }

    fn apply_destination_order_qos(&self, target: &mut cyclonedds_sys::dds_qos_t) {
        if let Some(destination_order) = &self.destination_order {
            ffi::dds_qos_set_destination_order(target, destination_order.as_ffi());
        }
    }

    fn apply_history_qos(&self, target: &mut cyclonedds_sys::dds_qos_t) {
        if let Some(history) = &self.history {
            let (kind, depth) = history.as_ffi();
            ffi::dds_qos_set_history(target, kind, depth);
        }
    }

    fn apply_resource_limits_qos(&self, target: &mut cyclonedds_sys::dds_qos_t) {
        if let Some(resource_limits) = &self.resource_limits {
            let (max_samples, max_instances, max_samples_per_instance) = resource_limits.as_ffi();
            ffi::dds_qos_set_resource_limits(
                target,
                max_samples,
                max_instances,
                max_samples_per_instance,
            );
        }
    }

    fn apply_entity_factory_qos(&self, target: &mut cyclonedds_sys::dds_qos_t) {
        if let Some(entity_factory) = &self.entity_factory {
            ffi::dds_qos_set_entity_factory(target, entity_factory.as_ffi());
        }
    }

    fn apply_writer_data_lifecycle_qos(&self, target: &mut cyclonedds_sys::dds_qos_t) {
        if let Some(writer_data_lifecycle) = &self.writer_data_lifecycle {
            ffi::dds_qos_set_writer_data_lifecycle(target, writer_data_lifecycle.as_ffi());
        }
    }

    fn apply_reader_data_lifecycle_qos(&self, target: &mut cyclonedds_sys::dds_qos_t) {
        if let Some(reader_data_lifecycle) = &self.reader_data_lifecycle {
            let (autopurge_nowriter_samples_delay, autopurge_disposed_samples_delay) =
                reader_data_lifecycle.as_ffi();
            ffi::dds_qos_set_reader_data_lifecycle(
                target,
                autopurge_nowriter_samples_delay,
                autopurge_disposed_samples_delay,
            );
        }
    }

    fn apply_entity_name_qos(&self, target: &mut cyclonedds_sys::dds_qos_t) {
        if let Some(entity_name) = &self.entity_name {
            let name = entity_name.as_ffi();
            ffi::dds_qos_set_entity_name(target, &name);
        }
    }
}

impl QoS {
    /// Creates a new [`QoS`] with no policies set.
    ///
    /// # Examples
    ///
    /// ```
    /// use cyclonedds::QoS;
    ///
    /// let qos = QoS::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the [`UserData`](policy::UserData) policy.
    ///
    /// # Examples
    ///
    /// ```
    /// use cyclonedds::QoS;
    /// use cyclonedds::qos::policy;
    ///
    /// let qos = QoS::new().with_user_data(policy::UserData {
    ///     value: b"v1.0".to_vec(),
    /// });
    /// ```
    #[must_use]
    pub fn with_user_data(mut self, user_data: policy::UserData) -> Self {
        self.user_data = Some(user_data);
        self
    }

    /// Sets the [`TopicData`](policy::TopicData) policy.
    ///
    /// # Examples
    ///
    /// ```
    /// use cyclonedds::QoS;
    /// use cyclonedds::qos::policy;
    ///
    /// let qos = QoS::new().with_topic_data(policy::TopicData {
    ///     value: b"sensor-schema".to_vec(),
    /// });
    /// ```
    #[must_use]
    pub fn with_topic_data(mut self, topic_data: policy::TopicData) -> Self {
        self.topic_data = Some(topic_data);
        self
    }

    /// Sets the [`GroupData`](policy::GroupData) policy.
    ///
    /// # Examples
    ///
    /// ```
    /// use cyclonedds::QoS;
    /// use cyclonedds::qos::policy;
    ///
    /// let qos = QoS::new().with_group_data(policy::GroupData {
    ///     value: b"group-a".to_vec(),
    /// });
    /// ```
    #[must_use]
    pub fn with_group_data(mut self, group_data: policy::GroupData) -> Self {
        self.group_data = Some(group_data);
        self
    }

    /// Sets the [`Durability`](policy::Durability) policy.
    ///
    /// # Examples
    ///
    /// ```
    /// use cyclonedds::QoS;
    /// use cyclonedds::qos::policy;
    ///
    /// let qos = QoS::new().with_durability(policy::Durability::TransientLocal);
    /// ```
    #[must_use]
    pub const fn with_durability(mut self, durability: policy::Durability) -> Self {
        self.durability = Some(durability);
        self
    }

    /// Sets the [`DurabilityService`](policy::DurabilityService) policy.
    ///
    /// # Examples
    ///
    /// ```
    /// use cyclonedds::qos::policy;
    /// use cyclonedds::{Duration, QoS};
    ///
    /// let qos = QoS::new().with_durability_service(policy::DurabilityService {
    ///     service_cleanup_delay: Duration::from_millis(200),
    ///     history: policy::History::KeepLast { depth: 10 },
    ///     resource_limits: policy::ResourceLimits {
    ///         max_samples: policy::ResourceLimit::Unlimited,
    ///         max_instances: policy::ResourceLimit::Unlimited,
    ///         max_samples_per_instance: policy::ResourceLimit::Unlimited,
    ///     },
    /// });
    /// ```
    #[must_use]
    pub const fn with_durability_service(
        mut self,
        durability_service: policy::DurabilityService,
    ) -> Self {
        self.durability_service = Some(durability_service);
        self
    }

    /// Sets the [`Presentation`](policy::Presentation) policy.
    ///
    /// # Examples
    ///
    /// ```
    /// use cyclonedds::QoS;
    /// use cyclonedds::qos::policy;
    ///
    /// let qos = QoS::new().with_presentation(policy::Presentation::Topic {
    ///     coherent_access: true,
    ///     ordered_access: true,
    /// });
    /// ```
    #[must_use]
    pub const fn with_presentation(mut self, presentation: policy::Presentation) -> Self {
        self.presentation = Some(presentation);
        self
    }

    /// Sets the [`Deadline`](policy::Deadline) policy.
    ///
    /// # Examples
    ///
    /// ```
    /// use cyclonedds::qos::policy;
    /// use cyclonedds::{Duration, QoS};
    ///
    /// let qos = QoS::new().with_deadline(policy::Deadline {
    ///     period: Duration::from_millis(100),
    /// });
    /// ```
    #[must_use]
    pub const fn with_deadline(mut self, deadline: policy::Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }

    /// Sets the [`LatencyBudget`](policy::LatencyBudget) policy.
    ///
    /// # Examples
    ///
    /// ```
    /// use cyclonedds::qos::policy;
    /// use cyclonedds::{Duration, QoS};
    ///
    /// let qos = QoS::new().with_latency_budget(policy::LatencyBudget {
    ///     duration: Duration::from_millis(10),
    /// });
    /// ```
    #[must_use]
    pub const fn with_latency_budget(mut self, latency_budget: policy::LatencyBudget) -> Self {
        self.latency_budget = Some(latency_budget);
        self
    }

    /// Sets the [`Ownership`](policy::Ownership) policy.
    ///
    /// # Examples
    ///
    /// ```
    /// use cyclonedds::QoS;
    /// use cyclonedds::qos::policy;
    ///
    /// let qos = QoS::new().with_ownership(policy::Ownership::Exclusive { strength: 10 });
    /// ```
    #[must_use]
    pub const fn with_ownership(mut self, ownership: policy::Ownership) -> Self {
        self.ownership = Some(ownership);
        self
    }

    /// Sets the [`Liveliness`](policy::Liveliness) policy.
    ///
    /// # Examples
    ///
    /// ```
    /// use cyclonedds::qos::policy;
    /// use cyclonedds::{Duration, QoS};
    ///
    /// let qos = QoS::new().with_liveliness(policy::Liveliness::Automatic {
    ///     lease_duration: Duration::from_secs(5),
    /// });
    /// ```
    #[must_use]
    pub const fn with_liveliness(mut self, liveliness: policy::Liveliness) -> Self {
        self.liveliness = Some(liveliness);
        self
    }

    /// Sets the [`TimeBasedFilter`](policy::TimeBasedFilter) policy.
    ///
    /// # Examples
    ///
    /// ```
    /// use cyclonedds::qos::policy;
    /// use cyclonedds::{Duration, QoS};
    ///
    /// let qos = QoS::new().with_time_based_filter(policy::TimeBasedFilter {
    ///     minimum_separation: Duration::from_millis(50),
    /// });
    /// ```
    #[must_use]
    pub const fn with_time_based_filter(
        mut self,
        time_based_filter: policy::TimeBasedFilter,
    ) -> Self {
        self.time_based_filter = Some(time_based_filter);
        self
    }

    /// Sets the [`Partition`](policy::Partition) policy.
    ///
    /// # Examples
    ///
    /// ```
    /// use cyclonedds::QoS;
    /// use cyclonedds::qos::policy;
    ///
    /// let qos = QoS::new().with_partition(policy::Partition {
    ///     partitions: vec!["sensors".to_string()],
    /// });
    /// ```
    #[must_use]
    pub fn with_partition(mut self, partition: policy::Partition) -> Self {
        self.partition = Some(partition);
        self
    }

    /// Sets the [`Reliability`](policy::Reliability) policy.
    ///
    /// # Examples
    ///
    /// ```
    /// use cyclonedds::qos::policy;
    /// use cyclonedds::{Duration, QoS};
    ///
    /// let qos = QoS::new().with_reliability(policy::Reliability::Reliable {
    ///     max_blocking_time: Duration::from_millis(100),
    /// });
    /// ```
    #[must_use]
    pub const fn with_reliability(mut self, reliability: policy::Reliability) -> Self {
        self.reliability = Some(reliability);
        self
    }

    /// Sets the [`TransportPriority`](policy::TransportPriority) policy.
    ///
    /// # Examples
    ///
    /// ```
    /// use cyclonedds::QoS;
    /// use cyclonedds::qos::policy;
    ///
    /// let qos = QoS::new().with_transport_priority(policy::TransportPriority { priority: 10 });
    /// ```
    #[must_use]
    pub const fn with_transport_priority(
        mut self,
        transport_priority: policy::TransportPriority,
    ) -> Self {
        self.transport_priority = Some(transport_priority);
        self
    }

    /// Sets the [`Lifespan`](policy::Lifespan) policy.
    ///
    /// # Examples
    ///
    /// ```
    /// use cyclonedds::qos::policy;
    /// use cyclonedds::{Duration, QoS};
    ///
    /// let qos = QoS::new().with_lifespan(policy::Lifespan {
    ///     duration: Duration::from_secs(30),
    /// });
    /// ```
    #[must_use]
    pub const fn with_lifespan(mut self, lifespan: policy::Lifespan) -> Self {
        self.lifespan = Some(lifespan);
        self
    }

    /// Sets the [`DestinationOrder`](policy::DestinationOrder) policy.
    ///
    /// # Examples
    ///
    /// ```
    /// use cyclonedds::QoS;
    /// use cyclonedds::qos::policy;
    ///
    /// let qos = QoS::new().with_destination_order(policy::DestinationOrder::BySourceTimestamp);
    /// ```
    #[must_use]
    pub const fn with_destination_order(
        mut self,
        destination_order: policy::DestinationOrder,
    ) -> Self {
        self.destination_order = Some(destination_order);
        self
    }

    /// Sets the [`History`](policy::History) policy.
    ///
    /// # Examples
    ///
    /// ```
    /// use cyclonedds::QoS;
    /// use cyclonedds::qos::policy;
    ///
    /// let qos = QoS::new().with_history(policy::History::KeepLast { depth: 10 });
    /// ```
    #[must_use]
    pub const fn with_history(mut self, history: policy::History) -> Self {
        self.history = Some(history);
        self
    }

    /// Sets the [`ResourceLimits`](policy::ResourceLimits) policy.
    ///
    /// # Examples
    ///
    /// ```
    /// use cyclonedds::QoS;
    /// use cyclonedds::qos::policy;
    ///
    /// let qos = QoS::new().with_resource_limits(policy::ResourceLimits {
    ///     max_samples: policy::ResourceLimit::Limited(1000),
    ///     max_instances: policy::ResourceLimit::Limited(100),
    ///     max_samples_per_instance: policy::ResourceLimit::Limited(10),
    /// });
    /// ```
    #[must_use]
    pub const fn with_resource_limits(mut self, resource_limits: policy::ResourceLimits) -> Self {
        self.resource_limits = Some(resource_limits);
        self
    }

    /// Sets the [`EntityFactory`](policy::EntityFactory) policy.
    ///
    /// # Examples
    ///
    /// ```
    /// use cyclonedds::QoS;
    /// use cyclonedds::qos::policy;
    ///
    /// let qos = QoS::new().with_entity_factory(policy::EntityFactory {
    ///     autoenable_created_entities: false,
    /// });
    /// ```
    #[must_use]
    pub const fn with_entity_factory(mut self, entity_factory: policy::EntityFactory) -> Self {
        self.entity_factory = Some(entity_factory);
        self
    }

    /// Sets the [`WriterDataLifecycle`](policy::WriterDataLifecycle) policy.
    ///
    /// # Examples
    ///
    /// ```
    /// use cyclonedds::QoS;
    /// use cyclonedds::qos::policy;
    ///
    /// let qos = QoS::new().with_writer_data_lifecycle(policy::WriterDataLifecycle {
    ///     autodispose_unregistered_instances: false,
    /// });
    /// ```
    #[must_use]
    pub const fn with_writer_data_lifecycle(
        mut self,
        writer_data_lifecycle: policy::WriterDataLifecycle,
    ) -> Self {
        self.writer_data_lifecycle = Some(writer_data_lifecycle);
        self
    }

    /// Sets the [`ReaderDataLifecycle`](policy::ReaderDataLifecycle) policy.
    ///
    /// # Examples
    ///
    /// ```
    /// use cyclonedds::qos::policy;
    /// use cyclonedds::{Duration, QoS};
    ///
    /// let qos = QoS::new().with_reader_data_lifecycle(policy::ReaderDataLifecycle {
    ///     autopurge_nowriter_samples_delay: Duration::from_secs(5),
    ///     autopurge_disposed_samples_delay: Duration::from_secs(1),
    /// });
    /// ```
    #[must_use]
    pub const fn with_reader_data_lifecycle(
        mut self,
        reader_data_lifecycle: policy::ReaderDataLifecycle,
    ) -> Self {
        self.reader_data_lifecycle = Some(reader_data_lifecycle);
        self
    }

    /// Sets the [`EntityName`](policy::EntityName) policy.
    ///
    /// # Examples
    ///
    /// ```
    /// use cyclonedds::QoS;
    /// use cyclonedds::qos::policy;
    ///
    /// let qos = QoS::new().with_entity_name(policy::EntityName {
    ///     name: "my_writer".to_string(),
    /// });
    /// ```
    #[must_use]
    pub fn with_entity_name(mut self, entity_name: policy::EntityName) -> Self {
        self.entity_name = Some(entity_name);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unsafe_code)]
    fn octetseq_as_slice(seq: &cyclonedds_sys::ddsi_octetseq) -> &[u8] {
        unsafe { std::slice::from_raw_parts(seq.value.cast_const(), seq.length as usize) }
    }

    #[allow(unsafe_code)]
    fn partition_as_vec(partition: &cyclonedds_sys::ddsi_stringseq) -> Vec<String> {
        unsafe { std::slice::from_raw_parts(partition.strs.cast_const(), partition.n as usize) }
            .iter()
            .map(|partition| {
                unsafe { std::ffi::CStr::from_ptr((*partition).cast_const()) }
                    .to_str()
                    .expect("partition name is valid UTF-8")
                    .to_string()
            })
            .collect()
    }

    #[allow(unsafe_code)]
    fn c_string_as_str(value: *const std::ffi::c_char) -> &'static str {
        unsafe { std::ffi::CStr::from_ptr(value) }
            .to_str()
            .expect("C string is valid UTF-8")
    }

    #[test]
    fn test_qos_set() {
        let user_data = policy::UserData {
            value: Vec::default(),
        };
        let topic_data = policy::TopicData {
            value: Vec::default(),
        };
        let group_data = policy::GroupData {
            value: Vec::default(),
        };
        let durability = policy::Durability::TransientLocal;
        let durability_service = policy::DurabilityService {
            service_cleanup_delay: crate::Duration::default(),
            history: policy::History::KeepAll,
            resource_limits: policy::ResourceLimits {
                max_samples: policy::ResourceLimit::Limited(1),
                max_instances: policy::ResourceLimit::Limited(1),
                max_samples_per_instance: policy::ResourceLimit::Limited(1),
            },
        };
        let presentation = policy::Presentation::Group {
            coherent_access: true,
            ordered_access: true,
        };
        let deadline = policy::Deadline {
            period: crate::Duration::default(),
        };
        let latency_budget = policy::LatencyBudget {
            duration: crate::Duration::default(),
        };
        let ownership = policy::Ownership::Shared;
        let liveliness = policy::Liveliness::Automatic {
            lease_duration: crate::Duration::default(),
        };
        let time_based_filter = policy::TimeBasedFilter {
            minimum_separation: crate::Duration::default(),
        };
        let partition = policy::Partition {
            partitions: Vec::default(),
        };
        let reliability = policy::Reliability::BestEffort;
        let transport_priority = policy::TransportPriority { priority: 0 };
        let lifespan = policy::Lifespan {
            duration: crate::Duration::default(),
        };
        let destination_order = policy::DestinationOrder::ByReceptionTimestamp;
        let history = policy::History::KeepAll;
        let resource_limits = policy::ResourceLimits {
            max_samples: policy::ResourceLimit::Limited(1),
            max_instances: policy::ResourceLimit::Limited(1),
            max_samples_per_instance: policy::ResourceLimit::Limited(1),
        };
        let entity_factory = policy::EntityFactory {
            autoenable_created_entities: false,
        };
        let writer_data_lifecycle = policy::WriterDataLifecycle {
            autodispose_unregistered_instances: true,
        };
        let reader_data_lifecycle = policy::ReaderDataLifecycle {
            autopurge_nowriter_samples_delay: crate::Duration::default(),
            autopurge_disposed_samples_delay: crate::Duration::default(),
        };
        let entity_name = policy::EntityName {
            name: String::default(),
        };

        let qos = QoS::new()
            .with_user_data(user_data.clone())
            .with_topic_data(topic_data.clone())
            .with_group_data(group_data.clone())
            .with_durability(durability)
            .with_durability_service(durability_service)
            .with_presentation(presentation)
            .with_deadline(deadline)
            .with_latency_budget(latency_budget)
            .with_ownership(ownership)
            .with_liveliness(liveliness)
            .with_time_based_filter(time_based_filter)
            .with_partition(partition.clone())
            .with_reliability(reliability)
            .with_transport_priority(transport_priority)
            .with_lifespan(lifespan)
            .with_destination_order(destination_order)
            .with_history(history)
            .with_resource_limits(resource_limits)
            .with_entity_factory(entity_factory)
            .with_writer_data_lifecycle(writer_data_lifecycle)
            .with_reader_data_lifecycle(reader_data_lifecycle)
            .with_entity_name(entity_name.clone());

        assert_eq!(qos.user_data, Some(user_data));
        assert_eq!(qos.topic_data, Some(topic_data));
        assert_eq!(qos.group_data, Some(group_data));
        assert_eq!(qos.durability, Some(durability));
        assert_eq!(qos.durability_service, Some(durability_service));
        assert_eq!(qos.presentation, Some(presentation));
        assert_eq!(qos.deadline, Some(deadline));
        assert_eq!(qos.latency_budget, Some(latency_budget));
        assert_eq!(qos.ownership, Some(ownership));
        assert_eq!(qos.liveliness, Some(liveliness));
        assert_eq!(qos.time_based_filter, Some(time_based_filter));
        assert_eq!(qos.partition, Some(partition));
        assert_eq!(qos.reliability, Some(reliability));
        assert_eq!(qos.transport_priority, Some(transport_priority));
        assert_eq!(qos.lifespan, Some(lifespan));
        assert_eq!(qos.destination_order, Some(destination_order));
        assert_eq!(qos.history, Some(history));
        assert_eq!(qos.resource_limits, Some(resource_limits));
        assert_eq!(qos.entity_factory, Some(entity_factory));
        assert_eq!(qos.writer_data_lifecycle, Some(writer_data_lifecycle));
        assert_eq!(qos.reader_data_lifecycle, Some(reader_data_lifecycle));
        assert_eq!(qos.entity_name, Some(entity_name));
    }

    #[test]
    fn test_qos_set_user_data() {
        let user_data = policy::UserData {
            value: b"user-data".to_vec(),
        };
        let qos = QoS::new().with_user_data(user_data.clone());
        assert_eq!(qos.user_data, Some(user_data.clone()));
        assert_eq!(octetseq_as_slice(&qos.as_ffi().user_data), user_data.value);
    }

    #[test]
    fn test_qos_set_topic_data() {
        let topic_data = policy::TopicData {
            value: b"topic-data".to_vec(),
        };
        let qos = QoS::new().with_topic_data(topic_data.clone());
        assert_eq!(qos.topic_data, Some(topic_data.clone()));
        assert_eq!(
            octetseq_as_slice(&qos.as_ffi().topic_data),
            topic_data.value
        );
    }

    #[test]
    fn test_qos_set_group_data() {
        let group_data = policy::GroupData {
            value: b"group-data".to_vec(),
        };
        let qos = QoS::new().with_group_data(group_data.clone());
        assert_eq!(qos.group_data, Some(group_data.clone()));
        assert_eq!(
            octetseq_as_slice(&qos.as_ffi().group_data),
            group_data.value
        );
    }

    #[test]
    fn test_qos_set_durability() {
        let durability = policy::Durability::Volatile;
        let qos = QoS::new().with_durability(durability);
        assert_eq!(qos.durability, Some(durability));
        assert_eq!(
            qos.as_ffi().durability.kind,
            cyclonedds_sys::dds_durability_kind_DDS_DURABILITY_VOLATILE
        );
        let durability = policy::Durability::TransientLocal;
        let qos = QoS::new().with_durability(durability);
        assert_eq!(qos.durability, Some(durability));
        assert_eq!(
            qos.as_ffi().durability.kind,
            cyclonedds_sys::dds_durability_kind_DDS_DURABILITY_TRANSIENT_LOCAL
        );
        let durability = policy::Durability::Transient;
        let qos = QoS::new().with_durability(durability);
        assert_eq!(qos.durability, Some(durability));
        assert_eq!(
            qos.as_ffi().durability.kind,
            cyclonedds_sys::dds_durability_kind_DDS_DURABILITY_TRANSIENT
        );
        let durability = policy::Durability::Persistent;
        let qos = QoS::new().with_durability(durability);
        assert_eq!(qos.durability, Some(durability));
        assert_eq!(
            qos.as_ffi().durability.kind,
            cyclonedds_sys::dds_durability_kind_DDS_DURABILITY_PERSISTENT
        );
    }

    #[test]
    fn test_qos_set_durability_service() {
        let durability_service = policy::DurabilityService {
            service_cleanup_delay: crate::Duration::INFINITE,
            history: policy::History::KeepAll,
            resource_limits: policy::ResourceLimits {
                max_samples: policy::ResourceLimit::Limited(1),
                max_instances: policy::ResourceLimit::Limited(1),
                max_samples_per_instance: policy::ResourceLimit::Limited(1),
            },
        };
        let qos = QoS::new().with_durability_service(durability_service);
        assert_eq!(qos.durability_service, Some(durability_service));
        let ffi_qos = qos.as_ffi();
        assert_eq!(
            ffi_qos.durability_service.service_cleanup_delay,
            crate::Duration::INFINITE.inner
        );
        assert_eq!(
            ffi_qos.durability_service.history.kind,
            cyclonedds_sys::dds_history_kind_DDS_HISTORY_KEEP_ALL
        );
        assert_eq!(ffi_qos.durability_service.history.depth, 0);
        assert_eq!(ffi_qos.durability_service.resource_limits.max_samples, 1);
        assert_eq!(ffi_qos.durability_service.resource_limits.max_instances, 1);
        assert_eq!(
            ffi_qos
                .durability_service
                .resource_limits
                .max_samples_per_instance,
            1
        );
    }

    #[test]
    fn test_qos_set_presentation() {
        let coherent_access = true;
        let ordered_access = true;

        let presentation = policy::Presentation::Instance {
            coherent_access,
            ordered_access,
        };
        let qos = QoS::new().with_presentation(presentation);
        assert_eq!(qos.presentation, Some(presentation));
        let ffi_qos = qos.as_ffi();
        assert_eq!(
            ffi_qos.presentation.access_scope,
            cyclonedds_sys::dds_presentation_access_scope_kind_DDS_PRESENTATION_INSTANCE
        );
        assert_eq!(ffi_qos.presentation.coherent_access, 1);
        assert_eq!(ffi_qos.presentation.ordered_access, 1);

        let presentation = policy::Presentation::Topic {
            coherent_access,
            ordered_access,
        };
        let qos = QoS::new().with_presentation(presentation);
        assert_eq!(qos.presentation, Some(presentation));
        let ffi_qos = qos.as_ffi();
        assert_eq!(
            ffi_qos.presentation.access_scope,
            cyclonedds_sys::dds_presentation_access_scope_kind_DDS_PRESENTATION_TOPIC
        );
        assert_eq!(ffi_qos.presentation.coherent_access, 1);
        assert_eq!(ffi_qos.presentation.ordered_access, 1);

        let presentation = policy::Presentation::Group {
            coherent_access,
            ordered_access,
        };
        let qos = QoS::new().with_presentation(presentation);
        assert_eq!(qos.presentation, Some(presentation));
        let ffi_qos = qos.as_ffi();
        assert_eq!(
            ffi_qos.presentation.access_scope,
            cyclonedds_sys::dds_presentation_access_scope_kind_DDS_PRESENTATION_GROUP
        );
        assert_eq!(ffi_qos.presentation.coherent_access, 1);
        assert_eq!(ffi_qos.presentation.ordered_access, 1);
    }

    #[test]
    fn test_qos_set_deadline() {
        let deadline = policy::Deadline {
            period: crate::Duration::INFINITE,
        };
        let qos = QoS::new().with_deadline(deadline);
        assert_eq!(qos.deadline, Some(deadline));
        assert_eq!(
            qos.as_ffi().deadline.deadline,
            crate::Duration::INFINITE.inner
        );
    }

    #[test]
    fn test_qos_set_latency_budget() {
        let latency_budget = policy::LatencyBudget {
            duration: crate::Duration::INFINITE,
        };
        let qos = QoS::new().with_latency_budget(latency_budget);
        assert_eq!(qos.latency_budget, Some(latency_budget));
        assert_eq!(
            qos.as_ffi().latency_budget.duration,
            crate::Duration::INFINITE.inner
        );
    }

    #[test]
    fn test_qos_set_ownership() {
        let ownership = policy::Ownership::Shared;
        let qos = QoS::new().with_ownership(ownership);
        assert_eq!(qos.ownership, Some(ownership));
        assert_eq!(
            qos.as_ffi().ownership.kind,
            cyclonedds_sys::dds_ownership_kind_DDS_OWNERSHIP_SHARED
        );

        let ownership = policy::Ownership::Exclusive { strength: 1 };
        let qos = QoS::new().with_ownership(ownership);
        assert_eq!(qos.ownership, Some(ownership));
        let ffi_qos = qos.as_ffi();
        assert_eq!(
            ffi_qos.ownership.kind,
            cyclonedds_sys::dds_ownership_kind_DDS_OWNERSHIP_EXCLUSIVE
        );
        assert_eq!(ffi_qos.ownership_strength.value, 1);
    }

    #[test]
    fn test_qos_set_liveliness() {
        let lease_duration = crate::Duration::INFINITE;

        let liveliness = policy::Liveliness::Automatic { lease_duration };
        let qos = QoS::new().with_liveliness(liveliness);
        assert_eq!(qos.liveliness, Some(liveliness));
        let ffi_qos = qos.as_ffi();
        assert_eq!(
            ffi_qos.liveliness.kind,
            cyclonedds_sys::dds_liveliness_kind_DDS_LIVELINESS_AUTOMATIC
        );
        assert_eq!(ffi_qos.liveliness.lease_duration, lease_duration.inner);

        let liveliness = policy::Liveliness::ManualByParticipant { lease_duration };
        let qos = QoS::new().with_liveliness(liveliness);
        assert_eq!(qos.liveliness, Some(liveliness));
        let ffi_qos = qos.as_ffi();
        assert_eq!(
            ffi_qos.liveliness.kind,
            cyclonedds_sys::dds_liveliness_kind_DDS_LIVELINESS_MANUAL_BY_PARTICIPANT
        );
        assert_eq!(ffi_qos.liveliness.lease_duration, lease_duration.inner);

        let liveliness = policy::Liveliness::ManualByTopic { lease_duration };
        let qos = QoS::new().with_liveliness(liveliness);
        assert_eq!(qos.liveliness, Some(liveliness));
        let ffi_qos = qos.as_ffi();
        assert_eq!(
            ffi_qos.liveliness.kind,
            cyclonedds_sys::dds_liveliness_kind_DDS_LIVELINESS_MANUAL_BY_TOPIC
        );
        assert_eq!(ffi_qos.liveliness.lease_duration, lease_duration.inner);
    }

    #[test]
    fn test_qos_set_time_based_filter() {
        let time_based_filter = policy::TimeBasedFilter {
            minimum_separation: crate::Duration::from_nanos(1000),
        };
        let qos = QoS::new().with_time_based_filter(time_based_filter);
        assert_eq!(qos.time_based_filter, Some(time_based_filter));
        assert_eq!(
            qos.as_ffi().time_based_filter.minimum_separation,
            crate::Duration::from_nanos(1000).inner
        );
    }

    #[test]
    fn test_qos_set_partition() {
        let partition = policy::Partition {
            partitions: vec!["A".to_string(), "B".to_string()],
        };
        let qos = QoS::new().with_partition(partition.clone());
        assert_eq!(qos.partition, Some(partition.clone()));
        assert_eq!(
            partition_as_vec(&qos.as_ffi().partition),
            partition.partitions
        );
    }

    #[test]
    #[should_panic = "unable to safely create std::ffi::CString from partition name"]
    fn test_qos_materialize_partition_with_invalid_name() {
        let partition = policy::Partition {
            partitions: vec!["A".to_string(), "\0".to_string()],
        };
        let qos = QoS::new().with_partition(partition.clone());
        let _ = qos.as_ffi();
    }

    #[test]
    fn test_qos_set_reliability() {
        let reliability = policy::Reliability::BestEffort;
        let qos = QoS::new().with_reliability(reliability);
        assert_eq!(qos.reliability, Some(reliability));
        let ffi_qos = qos.as_ffi();
        assert_eq!(
            ffi_qos.reliability.kind,
            cyclonedds_sys::dds_reliability_kind_DDS_RELIABILITY_BEST_EFFORT
        );
        assert_eq!(ffi_qos.reliability.max_blocking_time, 0);

        let reliability = policy::Reliability::Reliable {
            max_blocking_time: crate::Duration::INFINITE,
        };
        let qos = QoS::new().with_reliability(reliability);
        assert_eq!(qos.reliability, Some(reliability));
        let ffi_qos = qos.as_ffi();
        assert_eq!(
            ffi_qos.reliability.kind,
            cyclonedds_sys::dds_reliability_kind_DDS_RELIABILITY_RELIABLE
        );
        assert_eq!(
            ffi_qos.reliability.max_blocking_time,
            crate::Duration::INFINITE.inner
        );
    }

    #[test]
    fn test_qos_set_transport_priority() {
        let transport_priority = policy::TransportPriority { priority: 1 };
        let qos = QoS::new().with_transport_priority(transport_priority);
        assert_eq!(qos.transport_priority, Some(transport_priority));
        assert_eq!(qos.as_ffi().transport_priority.value, 1);
    }

    #[test]
    fn test_qos_set_lifespan() {
        let lifespan = policy::Lifespan {
            duration: crate::Duration::INFINITE,
        };
        let qos = QoS::new().with_lifespan(lifespan);
        assert_eq!(qos.lifespan, Some(lifespan));
        assert_eq!(
            qos.as_ffi().lifespan.duration,
            crate::Duration::INFINITE.inner
        );
    }

    #[test]
    fn test_qos_set_destination_order() {
        let destination_order = policy::DestinationOrder::ByReceptionTimestamp;
        let qos = QoS::new().with_destination_order(destination_order);
        assert_eq!(qos.destination_order, Some(destination_order));
        assert_eq!(
            qos.as_ffi().destination_order.kind,
            cyclonedds_sys::dds_destination_order_kind_DDS_DESTINATIONORDER_BY_RECEPTION_TIMESTAMP
        );

        let destination_order = policy::DestinationOrder::BySourceTimestamp;
        let qos = QoS::new().with_destination_order(destination_order);
        assert_eq!(qos.destination_order, Some(destination_order));
        assert_eq!(
            qos.as_ffi().destination_order.kind,
            cyclonedds_sys::dds_destination_order_kind_DDS_DESTINATIONORDER_BY_SOURCE_TIMESTAMP
        );
    }

    #[test]
    fn test_qos_set_history() {
        let history = policy::History::KeepAll;
        let qos = QoS::new().with_history(history);
        assert_eq!(qos.history, Some(history));
        let ffi_qos = qos.as_ffi();
        assert_eq!(
            ffi_qos.history.kind,
            cyclonedds_sys::dds_history_kind_DDS_HISTORY_KEEP_ALL
        );
        assert_eq!(ffi_qos.history.depth, 0);

        let history = policy::History::KeepLast { depth: 10 };
        let qos = QoS::new().with_history(history);
        assert_eq!(qos.history, Some(history));
        let ffi_qos = qos.as_ffi();
        assert_eq!(
            ffi_qos.history.kind,
            cyclonedds_sys::dds_history_kind_DDS_HISTORY_KEEP_LAST
        );
        assert_eq!(ffi_qos.history.depth, 10);
    }

    #[test]
    fn test_qos_set_resource_limits() {
        let resource_limits = policy::ResourceLimits {
            max_samples: policy::ResourceLimit::Limited(1),
            max_instances: policy::ResourceLimit::Limited(1),
            max_samples_per_instance: policy::ResourceLimit::Limited(1),
        };
        let qos = QoS::new().with_resource_limits(resource_limits);
        assert_eq!(qos.resource_limits, Some(resource_limits));
        let ffi_qos = qos.as_ffi();
        assert_eq!(ffi_qos.resource_limits.max_samples, 1);
        assert_eq!(ffi_qos.resource_limits.max_instances, 1);
        assert_eq!(ffi_qos.resource_limits.max_samples_per_instance, 1);
    }

    #[test]
    fn test_qos_set_entity_factory() {
        let entity_factory = policy::EntityFactory {
            autoenable_created_entities: true,
        };
        let qos = QoS::new().with_entity_factory(entity_factory);
        assert_eq!(qos.entity_factory, Some(entity_factory));
        assert_eq!(qos.as_ffi().entity_factory.autoenable_created_entities, 1);

        let entity_factory = policy::EntityFactory {
            autoenable_created_entities: false,
        };
        let qos = QoS::new().with_entity_factory(entity_factory);
        assert_eq!(qos.entity_factory, Some(entity_factory));
        assert_eq!(qos.as_ffi().entity_factory.autoenable_created_entities, 0);
    }

    #[test]
    fn test_qos_set_writer_data_lifecycle() {
        let writer_data_lifecycle = policy::WriterDataLifecycle {
            autodispose_unregistered_instances: true,
        };
        let qos = QoS::new().with_writer_data_lifecycle(writer_data_lifecycle);
        assert_eq!(qos.writer_data_lifecycle, Some(writer_data_lifecycle));
        assert_eq!(
            qos.as_ffi()
                .writer_data_lifecycle
                .autodispose_unregistered_instances,
            1
        );
    }

    #[test]
    fn test_qos_set_reader_data_lifecycle() {
        let reader_data_lifecycle = policy::ReaderDataLifecycle {
            autopurge_nowriter_samples_delay: crate::Duration::from_nanos(10_000),
            autopurge_disposed_samples_delay: crate::Duration::from_nanos(10_000),
        };
        let qos = QoS::new().with_reader_data_lifecycle(reader_data_lifecycle);
        assert_eq!(qos.reader_data_lifecycle, Some(reader_data_lifecycle));
        let ffi_qos = qos.as_ffi();
        assert_eq!(
            ffi_qos
                .reader_data_lifecycle
                .autopurge_nowriter_samples_delay,
            crate::Duration::from_nanos(10_000).inner
        );
        assert_eq!(
            ffi_qos
                .reader_data_lifecycle
                .autopurge_disposed_samples_delay,
            crate::Duration::from_nanos(10_000).inner
        );
    }

    #[test]
    fn test_qos_set_entity_name() {
        let entity_name = policy::EntityName {
            name: "my_entity".to_string(),
        };
        let qos = QoS::new().with_entity_name(entity_name.clone());
        assert_eq!(qos.entity_name, Some(entity_name.clone()));
        assert_eq!(
            c_string_as_str(qos.as_ffi().entity_name.cast_const()),
            entity_name.name
        );
    }

    #[test]
    #[should_panic = "unable to safely create std::ffi::CString from entity name"]
    fn test_qos_materialize_entity_name_with_invalid_name() {
        let entity_name = policy::EntityName {
            name: "\0".to_string(),
        };
        let qos = QoS::new().with_entity_name(entity_name);
        let _ = qos.as_ffi();
    }
}
