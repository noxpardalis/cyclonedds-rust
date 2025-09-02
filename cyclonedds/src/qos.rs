//!
//!

pub mod policy;

///
#[derive(Default)]
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
}

impl QoS {
    ///
    pub fn new() -> Self {
        Self::default()
    }

    ///
    pub fn with_user_data(mut self, user_data: policy::UserData) -> Self {
        self.user_data = Some(user_data);
        self
    }
    ///
    pub fn with_topic_data(mut self, topic_data: policy::TopicData) -> Self {
        self.topic_data = Some(topic_data);
        self
    }
    ///
    pub fn with_group_data(mut self, group_data: policy::GroupData) -> Self {
        self.group_data = Some(group_data);
        self
    }
    ///
    pub fn with_durability(mut self, durability: policy::Durability) -> Self {
        self.durability = Some(durability);
        self
    }
    ///
    pub fn with_durability_service(
        mut self,
        durability_service: policy::DurabilityService,
    ) -> Self {
        self.durability_service = Some(durability_service);
        self
    }
    ///
    pub fn with_presentation(mut self, presentation: policy::Presentation) -> Self {
        self.presentation = Some(presentation);
        self
    }
    ///
    pub fn with_deadline(mut self, deadline: policy::Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }
    ///
    pub fn with_latency_budget(mut self, latency_budget: policy::LatencyBudget) -> Self {
        self.latency_budget = Some(latency_budget);
        self
    }
    ///
    pub fn with_ownership(mut self, ownership: policy::Ownership) -> Self {
        self.ownership = Some(ownership);
        self
    }
    ///
    pub fn with_liveliness(mut self, liveliness: policy::Liveliness) -> Self {
        self.liveliness = Some(liveliness);
        self
    }
    ///
    pub fn with_time_based_filter(mut self, time_based_filter: policy::TimeBasedFilter) -> Self {
        self.time_based_filter = Some(time_based_filter);
        self
    }
    ///
    pub fn with_partition(mut self, partition: policy::Partition) -> Self {
        self.partition = Some(partition);
        self
    }
    ///
    pub fn with_reliability(mut self, reliability: policy::Reliability) -> Self {
        self.reliability = Some(reliability);
        self
    }
    ///
    pub fn with_transport_priority(
        mut self,
        transport_priority: policy::TransportPriority,
    ) -> Self {
        self.transport_priority = Some(transport_priority);
        self
    }
    ///
    pub fn with_lifespan(mut self, lifespan: policy::Lifespan) -> Self {
        self.lifespan = Some(lifespan);
        self
    }
    ///
    pub fn with_destination_order(mut self, destination_order: policy::DestinationOrder) -> Self {
        self.destination_order = Some(destination_order);
        self
    }
    ///
    pub fn with_history(mut self, history: policy::History) -> Self {
        self.history = Some(history);
        self
    }
    ///
    pub fn with_resource_limits(mut self, resource_limits: policy::ResourceLimits) -> Self {
        self.resource_limits = Some(resource_limits);
        self
    }
    ///
    pub fn with_entity_factory(mut self, entity_factory: policy::EntityFactory) -> Self {
        self.entity_factory = Some(entity_factory);
        self
    }
    ///
    pub fn with_writer_data_lifecycle(
        mut self,
        writer_data_lifecycle: policy::WriterDataLifecycle,
    ) -> Self {
        self.writer_data_lifecycle = Some(writer_data_lifecycle);
        self
    }
    ///
    pub fn with_reader_data_lifecycle(
        mut self,
        reader_data_lifecycle: policy::ReaderDataLifecycle,
    ) -> Self {
        self.reader_data_lifecycle = Some(reader_data_lifecycle);
        self
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
            .with_reader_data_lifecycle(reader_data_lifecycle.clone());

        assert_eq!(qos.user_data.unwrap(), user_data);
        assert_eq!(qos.topic_data.unwrap(), topic_data);
        assert_eq!(qos.group_data.unwrap(), group_data);
        assert_eq!(qos.durability.unwrap(), durability);
        assert_eq!(qos.durability_service.unwrap(), durability_service);
        assert_eq!(qos.presentation.unwrap(), presentation);
        assert_eq!(qos.deadline.unwrap(), deadline);
        assert_eq!(qos.latency_budget.unwrap(), latency_budget);
        assert_eq!(qos.ownership.unwrap(), ownership);
        assert_eq!(qos.liveliness.unwrap(), liveliness);
        assert_eq!(qos.time_based_filter.unwrap(), time_based_filter);
        assert_eq!(qos.partition.unwrap(), partition);
        assert_eq!(qos.reliability.unwrap(), reliability);
        assert_eq!(qos.transport_priority.unwrap(), transport_priority);
        assert_eq!(qos.lifespan.unwrap(), lifespan);
        assert_eq!(qos.destination_order.unwrap(), destination_order);
        assert_eq!(qos.history.unwrap(), history);
        assert_eq!(qos.resource_limits.unwrap(), resource_limits);
        assert_eq!(qos.entity_factory.unwrap(), entity_factory);
        assert_eq!(qos.writer_data_lifecycle.unwrap(), writer_data_lifecycle);
        assert_eq!(qos.reader_data_lifecycle.unwrap(), reader_data_lifecycle);
    }
}
