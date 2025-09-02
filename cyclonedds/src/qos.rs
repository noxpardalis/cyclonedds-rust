//!

pub mod policy;

use crate::internal::ffi;

///
#[derive(Default)]
pub struct QoS {
    pub(crate) inner: cyclonedds_sys::dds_qos_t,

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

impl QoS {
    ///
    pub fn new() -> Self {
        let mut qos = Self::default();
        ffi::ddsi_xqos_init(&mut qos.inner);
        qos
    }

    pub fn with_user_data(mut self, user_data: policy::UserData) -> Self {
        ffi::dds_qos_set_user_data(&mut self.inner, user_data.as_ffi());
        self.user_data = Some(user_data);
        self
    }

    pub fn with_topic_data(mut self, topic_data: policy::TopicData) -> Self {
        ffi::dds_qos_set_topic_data(&mut self.inner, topic_data.as_ffi());
        self.topic_data = Some(topic_data);
        self
    }

    pub fn with_group_data(mut self, group_data: policy::GroupData) -> Self {
        ffi::dds_qos_set_group_data(&mut self.inner, group_data.as_ffi());
        self.group_data = Some(group_data);
        self
    }

    ///
    pub fn with_durability(mut self, durability: policy::Durability) -> Self {
        ffi::dds_qos_set_durability(&mut self.inner, durability.as_ffi());
        self.durability = Some(durability);
        self
    }

    ///
    pub fn with_durability_service(
        mut self,
        durability_service: policy::DurabilityService,
    ) -> Self {
        let (
            service_cleanup_delay,
            history_kind,
            history_depth,
            max_samples,
            max_instances,
            max_samples_per_instance,
        ) = durability_service.as_ffi();

        ffi::dds_qos_set_durability_service(
            &mut self.inner,
            service_cleanup_delay,
            history_kind,
            history_depth,
            max_samples,
            max_instances,
            max_samples_per_instance,
        );
        self.durability_service = Some(durability_service);
        self
    }

    ///
    pub fn with_presentation(mut self, presentation: policy::Presentation) -> Self {
        let (access_scope, coherent_access, ordered_access) = presentation.as_ffi();
        ffi::dds_qos_set_presentation(
            &mut self.inner,
            access_scope,
            coherent_access,
            ordered_access,
        );
        self.presentation = Some(presentation);
        self
    }

    ///
    pub fn with_deadline(mut self, deadline: policy::Deadline) -> Self {
        let period = deadline.as_ffi();
        ffi::dds_qos_set_deadline(&mut self.inner, period);
        self.deadline = Some(deadline);
        self
    }

    ///
    pub fn with_latency_budget(mut self, latency_budget: policy::LatencyBudget) -> Self {
        let duration = latency_budget.as_ffi();
        ffi::dds_qos_set_latency_budget(&mut self.inner, duration);
        self.latency_budget = Some(latency_budget);
        self
    }

    ///
    pub fn with_ownership(mut self, ownership: policy::Ownership) -> Self {
        match &ownership {
            policy::Ownership::Shared => {
                ffi::dds_qos_set_ownership(
                    &mut self.inner,
                    cyclonedds_sys::dds_ownership_kind_DDS_OWNERSHIP_SHARED,
                );
            }
            policy::Ownership::Exclusive { strength } => {
                ffi::dds_qos_set_ownership(
                    &mut self.inner,
                    cyclonedds_sys::dds_ownership_kind_DDS_OWNERSHIP_EXCLUSIVE,
                );
                ffi::dds_qos_set_ownership_strength(&mut self.inner, *strength);
            }
        }

        self.ownership = Some(ownership);
        self
    }

    ///
    pub fn with_liveliness(mut self, liveliness: policy::Liveliness) -> Self {
        let (kind, lease_duration) = liveliness.as_ffi();
        ffi::dds_qos_set_liveliness(&mut self.inner, kind, lease_duration);
        self.liveliness = Some(liveliness);
        self
    }

    ///
    pub fn with_time_based_filter(mut self, time_based_filter: policy::TimeBasedFilter) -> Self {
        let minimum_separation = time_based_filter.as_ffi();
        ffi::dds_qos_set_time_based_filter(&mut self.inner, minimum_separation);
        self.time_based_filter = Some(time_based_filter);
        self
    }

    ///
    pub fn with_partition(mut self, partition: policy::Partition) -> Self {
        let partitions = partition.as_ffi();
        ffi::dds_qos_set_partition(&mut self.inner, partitions);
        self.partition = Some(partition);
        self
    }

    ///
    pub fn with_reliability(mut self, reliability: policy::Reliability) -> Self {
        let (kind, max_blocking_time) = reliability.as_ffi();
        ffi::dds_qos_set_reliability(&mut self.inner, kind, max_blocking_time);
        self.reliability = Some(reliability);
        self
    }

    ///
    pub fn with_transport_priority(
        mut self,
        transport_priority: policy::TransportPriority,
    ) -> Self {
        let lifespan = transport_priority.as_ffi();
        ffi::dds_qos_set_transport_priority(&mut self.inner, lifespan);
        self.transport_priority = Some(transport_priority);
        self
    }

    ///
    pub fn with_lifespan(mut self, lifespan: policy::Lifespan) -> Self {
        let duration = lifespan.as_ffi();
        ffi::dds_qos_set_lifespan(&mut self.inner, duration);
        self.lifespan = Some(lifespan);
        self
    }

    ///
    pub fn with_destination_order(mut self, destination_order: policy::DestinationOrder) -> Self {
        let kind = destination_order.as_ffi();
        ffi::dds_qos_set_destination_order(&mut self.inner, kind);
        self.destination_order = Some(destination_order);
        self
    }

    ///
    pub fn with_history(mut self, history: policy::History) -> Self {
        let (kind, depth) = history.as_ffi();
        ffi::dds_qos_set_history(&mut self.inner, kind, depth);
        self.history = Some(history);
        self
    }

    ///
    pub fn with_resource_limits(mut self, resource_limits: policy::ResourceLimits) -> Self {
        let (max_samples, max_instances, max_samples_per_instance) = resource_limits.as_ffi();
        ffi::dds_qos_set_resource_limits(
            &mut self.inner,
            max_samples,
            max_instances,
            max_samples_per_instance,
        );
        self.resource_limits = Some(resource_limits);
        self
    }

    ///
    pub fn with_entity_factory(mut self, entity_factory: policy::EntityFactory) -> Self {
        let auto_enable_created_entities = entity_factory.as_ffi();
        ffi::dds_qos_set_entity_factory(&mut self.inner, auto_enable_created_entities);
        self.entity_factory = Some(entity_factory);
        self
    }

    ///
    pub fn with_writer_data_lifecycle(
        mut self,
        writer_data_lifecycle: policy::WriterDataLifecycle,
    ) -> Self {
        let autodispose = writer_data_lifecycle.as_ffi();
        ffi::dds_qos_set_writer_data_lifecycle(&mut self.inner, autodispose);
        self.writer_data_lifecycle = Some(writer_data_lifecycle);
        self
    }

    ///
    pub fn with_reader_data_lifecycle(
        mut self,
        reader_data_lifecycle: policy::ReaderDataLifecycle,
    ) -> Self {
        let (autopurge_nowriter_samples_delay, autopurge_disposed_samples_delay) =
            reader_data_lifecycle.as_ffi();
        ffi::dds_qos_set_reader_data_lifecycle(
            &mut self.inner,
            autopurge_nowriter_samples_delay,
            autopurge_disposed_samples_delay,
        );
        self.reader_data_lifecycle = Some(reader_data_lifecycle);
        self
    }

    ///
    pub fn with_entity_name(mut self, entity_name: policy::EntityName) -> Self {
        let name = entity_name.as_ffi();
        ffi::dds_qos_set_entity_name(&mut self.inner, name);
        self.entity_name = Some(entity_name);
        self
    }
}

impl Drop for QoS {
    fn drop(&mut self) {
        ffi::ddsi_xqos_fini(&mut self.inner);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qos_set() {
        let user_data = policy::UserData {
            value: Default::default(),
        };
        let topic_data = policy::TopicData {
            value: Default::default(),
        };
        let group_data = policy::GroupData {
            value: Default::default(),
        };
        let durability = policy::Durability::TransientLocal;
        let durability_service = policy::DurabilityService {
            service_cleanup_delay: Default::default(),
            history: policy::History::KeepAll,
            max_samples: 1,
            max_instances: 1,
            max_samples_per_instance: 1,
        };
        let presentation = policy::Presentation::Group {
            coherent_access: true,
            ordered_access: true,
        };
        let deadline = policy::Deadline {
            period: Default::default(),
        };
        let latency_budget = policy::LatencyBudget {
            duration: Default::default(),
        };
        let ownership = policy::Ownership::Shared;
        let liveliness = policy::Liveliness::Automatic {
            lease_duration: Default::default(),
        };
        let time_based_filter = policy::TimeBasedFilter {
            minimum_separation: Default::default(),
        };
        let partition = policy::Partition {
            partitions: Default::default(),
        };
        let reliability = policy::Reliability::BestEffort;
        let transport_priority = policy::TransportPriority { priority: 0 };
        let lifespan = policy::Lifespan {
            duration: Default::default(),
        };
        let destination_order = policy::DestinationOrder::ByReceptionTimestamp;
        let history = policy::History::KeepAll;
        let resource_limits = policy::ResourceLimits {
            max_samples: 1,
            max_instances: 1,
            max_samples_per_instance: 1,
        };
        let entity_factory = policy::EntityFactory {
            autoenable_created_entities: false,
        };
        let writer_data_lifecycle = policy::WriterDataLifecycle {
            autodispose_unregistered_instances: true,
        };
        let reader_data_lifecycle = policy::ReaderDataLifecycle {
            autopurge_nowriter_samples_delay: Default::default(),
            autopurge_disposed_samples_delay: Default::default(),
        };
        let entity_name = policy::EntityName {
            name: Default::default(),
        };

        let qos = QoS::new()
            .with_user_data(user_data.clone())
            .with_topic_data(topic_data.clone())
            .with_group_data(group_data.clone())
            .with_durability(durability.clone())
            .with_durability_service(durability_service.clone())
            .with_presentation(presentation.clone())
            .with_deadline(deadline.clone())
            .with_latency_budget(latency_budget.clone())
            .with_ownership(ownership.clone())
            .with_liveliness(liveliness.clone())
            .with_time_based_filter(time_based_filter.clone())
            .with_partition(partition.clone())
            .with_reliability(reliability.clone())
            .with_transport_priority(transport_priority.clone())
            .with_lifespan(lifespan.clone())
            .with_destination_order(destination_order.clone())
            .with_history(history.clone())
            .with_resource_limits(resource_limits.clone())
            .with_entity_factory(entity_factory.clone())
            .with_writer_data_lifecycle(writer_data_lifecycle.clone())
            .with_reader_data_lifecycle(reader_data_lifecycle.clone())
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
            value: Default::default(),
        };
        let qos = QoS::new().with_user_data(user_data.clone());
        assert_eq!(qos.user_data, Some(user_data));
    }

    #[test]
    fn test_qos_set_topic_data() {
        let topic_data = policy::TopicData {
            value: Default::default(),
        };
        let qos = QoS::new().with_topic_data(topic_data.clone());
        assert_eq!(qos.topic_data, Some(topic_data));
    }

    #[test]
    fn test_qos_set_group_data() {
        let group_data = policy::GroupData {
            value: Default::default(),
        };
        let qos = QoS::new().with_group_data(group_data.clone());
        assert_eq!(qos.group_data, Some(group_data));
    }

    #[test]
    fn test_qos_set_durability() {
        let durability = policy::Durability::Volatile;
        let qos = QoS::new().with_durability(durability.clone());
        assert_eq!(qos.durability, Some(durability));
        let durability = policy::Durability::TransientLocal;
        let qos = QoS::new().with_durability(durability.clone());
        assert_eq!(qos.durability, Some(durability));
        let durability = policy::Durability::Transient;
        let qos = QoS::new().with_durability(durability.clone());
        assert_eq!(qos.durability, Some(durability));
        let durability = policy::Durability::Persistent;
        let qos = QoS::new().with_durability(durability.clone());
        assert_eq!(qos.durability, Some(durability));
    }

    #[test]
    fn test_qos_set_durability_service() {
        let durability_service = policy::DurabilityService {
            service_cleanup_delay: crate::Duration::INFINITE,
            history: policy::History::KeepAll,
            max_samples: 1,
            max_instances: 1,
            max_samples_per_instance: 1,
        };
        let qos = QoS::new().with_durability_service(durability_service.clone());
        assert_eq!(qos.durability_service, Some(durability_service));
    }

    #[test]
    fn test_qos_set_presentation() {
        let coherent_access = true;
        let ordered_access = true;

        let presentation = policy::Presentation::Instance {
            coherent_access,
            ordered_access,
        };
        let qos = QoS::new().with_presentation(presentation.clone());
        assert_eq!(qos.presentation, Some(presentation));

        let presentation = policy::Presentation::Topic {
            coherent_access,
            ordered_access,
        };
        let qos = QoS::new().with_presentation(presentation.clone());
        assert_eq!(qos.presentation, Some(presentation));

        let presentation = policy::Presentation::Group {
            coherent_access,
            ordered_access,
        };
        let qos = QoS::new().with_presentation(presentation.clone());
        assert_eq!(qos.presentation, Some(presentation));
    }

    #[test]
    fn test_qos_set_deadline() {
        let deadline = policy::Deadline {
            period: crate::Duration::INFINITE,
        };
        let qos = QoS::new().with_deadline(deadline.clone());
        assert_eq!(qos.deadline, Some(deadline));
    }

    #[test]
    fn test_qos_set_latency_budget() {
        let latency_budget = policy::LatencyBudget {
            duration: crate::Duration::INFINITE,
        };
        let qos = QoS::new().with_latency_budget(latency_budget.clone());
        assert_eq!(qos.latency_budget, Some(latency_budget));
    }

    #[test]
    fn test_qos_set_ownership() {
        let ownership = policy::Ownership::Shared;
        let qos = QoS::new().with_ownership(ownership.clone());
        assert_eq!(qos.ownership, Some(ownership));

        let ownership = policy::Ownership::Exclusive { strength: 1 };
        let qos = QoS::new().with_ownership(ownership.clone());
        assert_eq!(qos.ownership, Some(ownership));
    }

    #[test]
    fn test_qos_set_liveliness() {
        let lease_duration = crate::Duration::INFINITE;

        let liveliness = policy::Liveliness::Automatic { lease_duration };
        let qos = QoS::new().with_liveliness(liveliness.clone());
        assert_eq!(qos.liveliness, Some(liveliness));

        let liveliness = policy::Liveliness::ManualByParticipant { lease_duration };
        let qos = QoS::new().with_liveliness(liveliness.clone());
        assert_eq!(qos.liveliness, Some(liveliness));

        let liveliness = policy::Liveliness::ManualByTopic { lease_duration };
        let qos = QoS::new().with_liveliness(liveliness.clone());
        assert_eq!(qos.liveliness, Some(liveliness));
    }

    #[test]
    fn test_qos_set_time_based_filter() {
        let time_based_filter = policy::TimeBasedFilter {
            minimum_separation: crate::Duration::from_nanos(1000),
        };
        let qos = QoS::new().with_time_based_filter(time_based_filter.clone());
        assert_eq!(qos.time_based_filter, Some(time_based_filter));
    }

    #[test]
    fn test_qos_set_partition() {
        let partition = policy::Partition {
            partitions: vec!["A".to_string(), "B".to_string()],
        };
        let qos = QoS::new().with_partition(partition.clone());
        assert_eq!(qos.partition, Some(partition));
    }

    #[test]
    fn test_qos_set_reliability() {
        let reliability = policy::Reliability::BestEffort;
        let qos = QoS::new().with_reliability(reliability.clone());
        assert_eq!(qos.reliability, Some(reliability));

        let reliability = policy::Reliability::Reliable {
            max_blocking_time: crate::Duration::INFINITE,
        };
        let qos = QoS::new().with_reliability(reliability.clone());
        assert_eq!(qos.reliability, Some(reliability));
    }

    #[test]
    fn test_qos_set_transport_priority() {
        let transport_priority = policy::TransportPriority { priority: 1 };
        let qos = QoS::new().with_transport_priority(transport_priority.clone());
        assert_eq!(qos.transport_priority, Some(transport_priority));
    }

    #[test]
    fn test_qos_set_lifespan() {
        let lifespan = policy::Lifespan {
            duration: crate::Duration::INFINITE,
        };
        let qos = QoS::new().with_lifespan(lifespan.clone());
        assert_eq!(qos.lifespan, Some(lifespan));
    }

    #[test]
    fn test_qos_set_destination_order() {
        let destination_order = policy::DestinationOrder::ByReceptionTimestamp;
        let qos = QoS::new().with_destination_order(destination_order.clone());
        assert_eq!(qos.destination_order, Some(destination_order));

        let destination_order = policy::DestinationOrder::BySourceTimestamp;
        let qos = QoS::new().with_destination_order(destination_order.clone());
        assert_eq!(qos.destination_order, Some(destination_order));
    }

    #[test]
    fn test_qos_set_history() {
        let history = policy::History::KeepAll;
        let qos = QoS::new().with_history(history.clone());
        assert_eq!(qos.history, Some(history));

        let history = policy::History::KeepLast { depth: 10 };
        let qos = QoS::new().with_history(history.clone());
        assert_eq!(qos.history, Some(history));
    }

    #[test]
    fn test_qos_set_resource_limits() {
        let resource_limits = policy::ResourceLimits {
            max_samples: 1,
            max_instances: 1,
            max_samples_per_instance: 1,
        };
        let qos = QoS::new().with_resource_limits(resource_limits.clone());
        assert_eq!(qos.resource_limits, Some(resource_limits));
    }

    #[test]
    fn test_qos_set_entity_factory() {
        let entity_factory = policy::EntityFactory {
            autoenable_created_entities: true,
        };
        let qos = QoS::new().with_entity_factory(entity_factory.clone());
        assert_eq!(qos.entity_factory, Some(entity_factory));
    }

    #[test]
    fn test_qos_set_writer_data_lifecycle() {
        let writer_data_lifecycle = policy::WriterDataLifecycle {
            autodispose_unregistered_instances: true,
        };
        let qos = QoS::new().with_writer_data_lifecycle(writer_data_lifecycle.clone());
        assert_eq!(qos.writer_data_lifecycle, Some(writer_data_lifecycle));
    }

    #[test]
    fn test_qos_set_reader_data_lifecycle() {
        let reader_data_lifecycle = policy::ReaderDataLifecycle {
            autopurge_nowriter_samples_delay: crate::Duration::from_nanos(10_000),
            autopurge_disposed_samples_delay: crate::Duration::from_nanos(10_000),
        };
        let qos = QoS::new().with_reader_data_lifecycle(reader_data_lifecycle.clone());
        assert_eq!(qos.reader_data_lifecycle, Some(reader_data_lifecycle));
    }

    #[test]
    fn test_qos_set_entity_name() {
        let entity_name = policy::EntityName {
            name: "my_entity".to_string(),
        };
        let qos = QoS::new().with_entity_name(entity_name.clone());
        assert_eq!(qos.entity_name, Some(entity_name));
    }
}
