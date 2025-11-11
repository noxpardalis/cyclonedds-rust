//! All entities have a set of status conditions (following the DCPS spec). This
//! modules exposes the set of status conditions as a set of bit-flags.

bitflags::bitflags! {
    /// Flags for specifying the set of statuses that are of interest.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Status: cyclonedds_sys::dds_status_id_t {
        /// Another topic exists with the same name but with different
        /// characteristics.
        const InconsistentTopic =
            1 << cyclonedds_sys::dds_status_id_DDS_INCONSISTENT_TOPIC_STATUS_ID;
        /// The deadline that the writer has committed through its deadline QoS
        /// policy was not respected for a specific instance.
        const OfferedDeadlineMissed =
            1 << cyclonedds_sys::dds_status_id_DDS_OFFERED_DEADLINE_MISSED_STATUS_ID;
        /// The deadline that the reader was expecting through its deadline QoS
        /// policy was not respected for a specific instance.
        const RequestedDeadlineMissed =
            1 << cyclonedds_sys::dds_status_id_DDS_REQUESTED_DEADLINE_MISSED_STATUS_ID;
        /// A QoS policy setting was incompatible with what was requested.
        const OfferedIncompatibleQoS =
            1 << cyclonedds_sys::dds_status_id_DDS_OFFERED_INCOMPATIBLE_QOS_STATUS_ID;
        /// A QoS policy setting was incompatible with what is offered.
        const RequestedIncompatibleQoS =
            1 << cyclonedds_sys::dds_status_id_DDS_REQUESTED_INCOMPATIBLE_QOS_STATUS_ID;
        /// A sample has been lost (never received).
        const SampleLost =
            1 << cyclonedds_sys::dds_status_id_DDS_SAMPLE_LOST_STATUS_ID;
        /// A (received) sample has been rejected.
        const SampleRejected =
            1 << cyclonedds_sys::dds_status_id_DDS_SAMPLE_REJECTED_STATUS_ID;
        /// New information is available in some of the data readers of a subscriber.
        const DataOnReaders =
            1 << cyclonedds_sys::dds_status_id_DDS_DATA_ON_READERS_STATUS_ID;
        /// New information is available in a data reader.
        const DataAvailable =
            1 << cyclonedds_sys::dds_status_id_DDS_DATA_AVAILABLE_STATUS_ID;
        /// The liveliness that the Writer has committed through its liveliness
        /// QoS policy was not respected; thus readers will consider the writer
        /// as no longer "alive".
        const LivelinessLost =
            1 << cyclonedds_sys::dds_status_id_DDS_LIVELINESS_LOST_STATUS_ID;
        /// The liveliness of one or more writers, that were writing instances
        /// read through the readers has changed. Some writers have become
        /// "alive" or "not alive".
        const LivelinessChanged =
            1 << cyclonedds_sys::dds_status_id_DDS_LIVELINESS_CHANGED_STATUS_ID;
        /// The writer has found a reader that matches the topic and has a
        /// compatible QoS.
        const PublicationMatched =
            1 << cyclonedds_sys::dds_status_id_DDS_PUBLICATION_MATCHED_STATUS_ID;
        /// The reader has found a writer that matches the topic and has a
        /// compatible QoS.
        const SubscriptionMatched =
            1 << cyclonedds_sys::dds_status_id_DDS_SUBSCRIPTION_MATCHED_STATUS_ID;
    }
}

///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QoSPolicyId {
    ///
    Invalid,
    ///
    UserData,
    ///
    Durability,
    ///
    Presentation,
    ///
    Deadline,
    ///
    LatencyBudget,
    ///
    Ownership,
    ///
    OwnershipStrength,
    ///
    Liveliness,
    ///
    TimeBasedFilter,
    ///
    Partition,
    ///
    Reliability,
    ///
    DestinationOrder,
    ///
    History,
    ///
    ResourceLimits,
    ///
    EntityFactory,
    ///
    WriterDataLifecycle,
    ///
    ReaderDataLifecycle,
    ///
    TopicData,
    ///
    GroupData,
    ///
    TransportPriority,
    ///
    Lifespan,
    ///
    DurabilityService,
    ///
    Property,
    ///
    TypeConsistencyEnforcement,
    ///
    DataRepresentation,
}

impl From<cyclonedds_sys::dds_qos_policy_id_t> for QoSPolicyId {
    fn from(value: cyclonedds_sys::dds_qos_policy_id_t) -> Self {
        match value {
            cyclonedds_sys::dds_qos_policy_id_DDS_INVALID_QOS_POLICY_ID => Self::Invalid,
            cyclonedds_sys::dds_qos_policy_id_DDS_USERDATA_QOS_POLICY_ID => Self::UserData,
            cyclonedds_sys::dds_qos_policy_id_DDS_DURABILITY_QOS_POLICY_ID => Self::Durability,
            cyclonedds_sys::dds_qos_policy_id_DDS_PRESENTATION_QOS_POLICY_ID => Self::Presentation,
            cyclonedds_sys::dds_qos_policy_id_DDS_DEADLINE_QOS_POLICY_ID => Self::Deadline,
            cyclonedds_sys::dds_qos_policy_id_DDS_LATENCYBUDGET_QOS_POLICY_ID => {
                Self::LatencyBudget
            }
            cyclonedds_sys::dds_qos_policy_id_DDS_OWNERSHIP_QOS_POLICY_ID => Self::Ownership,
            cyclonedds_sys::dds_qos_policy_id_DDS_OWNERSHIPSTRENGTH_QOS_POLICY_ID => {
                Self::OwnershipStrength
            }
            cyclonedds_sys::dds_qos_policy_id_DDS_LIVELINESS_QOS_POLICY_ID => Self::Liveliness,
            cyclonedds_sys::dds_qos_policy_id_DDS_TIMEBASEDFILTER_QOS_POLICY_ID => {
                Self::TimeBasedFilter
            }
            cyclonedds_sys::dds_qos_policy_id_DDS_PARTITION_QOS_POLICY_ID => Self::Partition,
            cyclonedds_sys::dds_qos_policy_id_DDS_RELIABILITY_QOS_POLICY_ID => Self::Reliability,
            cyclonedds_sys::dds_qos_policy_id_DDS_DESTINATIONORDER_QOS_POLICY_ID => {
                Self::DestinationOrder
            }
            cyclonedds_sys::dds_qos_policy_id_DDS_HISTORY_QOS_POLICY_ID => Self::History,
            cyclonedds_sys::dds_qos_policy_id_DDS_RESOURCELIMITS_QOS_POLICY_ID => {
                Self::ResourceLimits
            }
            cyclonedds_sys::dds_qos_policy_id_DDS_ENTITYFACTORY_QOS_POLICY_ID => {
                Self::EntityFactory
            }
            cyclonedds_sys::dds_qos_policy_id_DDS_WRITERDATALIFECYCLE_QOS_POLICY_ID => {
                Self::WriterDataLifecycle
            }
            cyclonedds_sys::dds_qos_policy_id_DDS_READERDATALIFECYCLE_QOS_POLICY_ID => {
                Self::ReaderDataLifecycle
            }
            cyclonedds_sys::dds_qos_policy_id_DDS_TOPICDATA_QOS_POLICY_ID => Self::TopicData,
            cyclonedds_sys::dds_qos_policy_id_DDS_GROUPDATA_QOS_POLICY_ID => Self::GroupData,
            cyclonedds_sys::dds_qos_policy_id_DDS_TRANSPORTPRIORITY_QOS_POLICY_ID => {
                Self::TransportPriority
            }
            cyclonedds_sys::dds_qos_policy_id_DDS_LIFESPAN_QOS_POLICY_ID => Self::Lifespan,
            cyclonedds_sys::dds_qos_policy_id_DDS_DURABILITYSERVICE_QOS_POLICY_ID => {
                Self::DurabilityService
            }
            cyclonedds_sys::dds_qos_policy_id_DDS_PROPERTY_QOS_POLICY_ID => Self::Property,
            cyclonedds_sys::dds_qos_policy_id_DDS_TYPE_CONSISTENCY_ENFORCEMENT_QOS_POLICY_ID => {
                Self::TypeConsistencyEnforcement
            }
            cyclonedds_sys::dds_qos_policy_id_DDS_DATA_REPRESENTATION_QOS_POLICY_ID => {
                Self::DataRepresentation
            }
            value => unimplemented!(
                "unsupported value: {value} in conversion to {}",
                std::any::type_name::<Self>()
            ),
        }
    }
}

///
pub struct Count {
    ///
    pub value: u32,
    ///
    pub delta: i32,
}

///
pub struct InconsistentTopic {
    ///
    pub total: Count,
}

///
pub struct LivelinessLost {
    ///
    pub total: Count,
}

///
pub struct OfferedDeadlineMissed {
    ///
    pub total: Count,
    ///
    pub last_instance_handle: crate::entity::InstanceHandle,
}

///
pub struct OfferedIncompatibleQoS {
    ///
    pub total: Count,
    ///
    pub last_policy_id: QoSPolicyId,
}

///
pub struct PublicationMatched {
    ///
    pub total: Count,
    ///
    pub current: Count,
    ///
    pub last_subscription_handle: crate::entity::InstanceHandle,
}

///
pub struct SampleLost {
    ///
    pub total: Count,
}

///
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SampleRejectedReason {
    /// TODO How can sample not rejected be a valid status for the sample rejected status?
    /// should this be modeled as an Option in samplerejected?
    NotRejected,
    ///
    RejectedByInstancesLimit,
    ///
    RejectedBySamplesLimit,
    ///
    RejectedBySamplesPerInstanceLimit,
}

impl From<cyclonedds_sys::dds_sample_rejected_status_kind> for SampleRejectedReason {
    fn from(reason: cyclonedds_sys::dds_sample_rejected_status_kind) -> Self {
        match reason {
            cyclonedds_sys::dds_sample_rejected_status_kind_DDS_NOT_REJECTED => Self::NotRejected,
            cyclonedds_sys::dds_sample_rejected_status_kind_DDS_REJECTED_BY_INSTANCES_LIMIT => Self::RejectedByInstancesLimit,
            cyclonedds_sys::dds_sample_rejected_status_kind_DDS_REJECTED_BY_SAMPLES_LIMIT => Self::RejectedBySamplesLimit,
            cyclonedds_sys::dds_sample_rejected_status_kind_DDS_REJECTED_BY_SAMPLES_PER_INSTANCE_LIMIT => Self::RejectedBySamplesPerInstanceLimit,
            value => unimplemented!("unsupported value: {value} in conversion to {}", std::any::type_name::<Self>())
        }
    }
}

///
pub struct SampleRejected {
    ///
    pub total: Count,
    ///
    pub last_reason: SampleRejectedReason,
    ///
    pub last_instance_handle: crate::entity::InstanceHandle,
}

///
pub struct RequestedDeadlineMissed {
    ///
    pub total: Count,
    ///
    pub last_instance_handle: crate::entity::InstanceHandle,
}

///
pub struct RequestedIncompatibleQoS {
    ///
    pub total: Count,
    ///
    pub last_policy_id: QoSPolicyId,
}

///
pub struct SubscriptionMatched {
    ///
    pub total: Count,
    ///
    pub current: Count,
    ///
    pub last_publication_handle: crate::entity::InstanceHandle,
}

///
pub struct LivelinessChanged {
    ///
    pub alive: Count,
    ///
    pub not_alive: Count,
    ///
    pub last_publication_handle: crate::entity::InstanceHandle,
}

impl From<cyclonedds_sys::dds_inconsistent_topic_status> for InconsistentTopic {
    fn from(status: cyclonedds_sys::dds_inconsistent_topic_status) -> Self {
        let total = Count {
            value: status.total_count,
            delta: status.total_count_change,
        };

        Self { total }
    }
}

impl From<cyclonedds_sys::dds_liveliness_lost_status_t> for LivelinessLost {
    fn from(status: cyclonedds_sys::dds_liveliness_lost_status_t) -> Self {
        let total = Count {
            value: status.total_count,
            delta: status.total_count_change,
        };
        Self { total }
    }
}

impl From<cyclonedds_sys::dds_offered_deadline_missed_status_t> for OfferedDeadlineMissed {
    fn from(status: cyclonedds_sys::dds_offered_deadline_missed_status_t) -> Self {
        let total = Count {
            value: status.total_count,
            delta: status.total_count_change,
        };
        let last_instance_handle = crate::entity::InstanceHandle {
            inner: status.last_instance_handle,
        };

        Self {
            total,
            last_instance_handle,
        }
    }
}

impl From<cyclonedds_sys::dds_offered_incompatible_qos_status_t> for OfferedIncompatibleQoS {
    fn from(status: cyclonedds_sys::dds_offered_incompatible_qos_status_t) -> Self {
        let total = Count {
            value: status.total_count,
            delta: status.total_count_change,
        };
        let last_policy_id = status.last_policy_id.into();

        Self {
            total,
            last_policy_id,
        }
    }
}

impl From<cyclonedds_sys::dds_publication_matched_status_t> for PublicationMatched {
    fn from(status: cyclonedds_sys::dds_publication_matched_status_t) -> Self {
        let total = Count {
            value: status.total_count,
            delta: status.total_count_change,
        };
        let current = Count {
            value: status.current_count,
            delta: status.current_count_change,
        };
        let last_subscription_handle = crate::entity::InstanceHandle {
            inner: status.last_subscription_handle,
        };
        Self {
            total,
            current,
            last_subscription_handle,
        }
    }
}

impl From<cyclonedds_sys::dds_sample_lost_status_t> for SampleLost {
    fn from(status: cyclonedds_sys::dds_sample_lost_status_t) -> Self {
        let total = Count {
            value: status.total_count,
            delta: status.total_count_change,
        };
        Self { total }
    }
}

impl From<cyclonedds_sys::dds_sample_rejected_status_t> for SampleRejected {
    fn from(status: cyclonedds_sys::dds_sample_rejected_status_t) -> Self {
        let total = Count {
            value: status.total_count,
            delta: status.total_count_change,
        };
        let last_reason = status.last_reason.into();
        let last_instance_handle = crate::entity::InstanceHandle {
            inner: status.last_instance_handle,
        };
        Self {
            total,
            last_reason,
            last_instance_handle,
        }
    }
}

impl From<cyclonedds_sys::dds_liveliness_changed_status_t> for LivelinessChanged {
    fn from(status: cyclonedds_sys::dds_liveliness_changed_status_t) -> Self {
        let alive = Count {
            value: status.alive_count,
            delta: status.alive_count_change,
        };
        let not_alive = Count {
            value: status.not_alive_count,
            delta: status.not_alive_count_change,
        };

        let last_publication_handle = crate::entity::InstanceHandle {
            inner: status.last_publication_handle,
        };
        Self {
            alive,
            not_alive,
            last_publication_handle,
        }
    }
}

impl From<cyclonedds_sys::dds_requested_deadline_missed_status_t> for RequestedDeadlineMissed {
    fn from(status: cyclonedds_sys::dds_requested_deadline_missed_status_t) -> Self {
        let total = Count {
            value: status.total_count,
            delta: status.total_count_change,
        };
        let last_instance_handle = crate::entity::InstanceHandle {
            inner: status.last_instance_handle,
        };
        Self {
            total,
            last_instance_handle,
        }
    }
}

impl From<cyclonedds_sys::dds_requested_incompatible_qos_status_t> for RequestedIncompatibleQoS {
    fn from(status: cyclonedds_sys::dds_requested_incompatible_qos_status_t) -> Self {
        let total = Count {
            value: status.total_count,
            delta: status.total_count_change,
        };
        let last_policy_id = status.last_policy_id.into();
        Self {
            total,
            last_policy_id,
        }
    }
}

impl From<cyclonedds_sys::dds_subscription_matched_status_t> for SubscriptionMatched {
    fn from(status: cyclonedds_sys::dds_subscription_matched_status_t) -> Self {
        let total = Count {
            value: status.total_count,
            delta: status.total_count_change,
        };
        let current = Count {
            value: status.current_count,
            delta: status.current_count_change,
        };
        let last_publication_handle = crate::entity::InstanceHandle {
            inner: status.last_publication_handle,
        };

        Self {
            total,
            current,
            last_publication_handle,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qos_policy_id_conversion() {
        let result = QoSPolicyId::from(cyclonedds_sys::dds_qos_policy_id_DDS_INVALID_QOS_POLICY_ID);
        assert_eq!(result, QoSPolicyId::Invalid);
        let result =
            QoSPolicyId::from(cyclonedds_sys::dds_qos_policy_id_DDS_USERDATA_QOS_POLICY_ID);
        assert_eq!(result, QoSPolicyId::UserData);
        let result =
            QoSPolicyId::from(cyclonedds_sys::dds_qos_policy_id_DDS_DURABILITY_QOS_POLICY_ID);
        assert_eq!(result, QoSPolicyId::Durability);
        let result =
            QoSPolicyId::from(cyclonedds_sys::dds_qos_policy_id_DDS_PRESENTATION_QOS_POLICY_ID);
        assert_eq!(result, QoSPolicyId::Presentation);
        let result =
            QoSPolicyId::from(cyclonedds_sys::dds_qos_policy_id_DDS_DEADLINE_QOS_POLICY_ID);
        assert_eq!(result, QoSPolicyId::Deadline);
        let result =
            QoSPolicyId::from(cyclonedds_sys::dds_qos_policy_id_DDS_LATENCYBUDGET_QOS_POLICY_ID);
        assert_eq!(result, QoSPolicyId::LatencyBudget);
        let result =
            QoSPolicyId::from(cyclonedds_sys::dds_qos_policy_id_DDS_OWNERSHIP_QOS_POLICY_ID);
        assert_eq!(result, QoSPolicyId::Ownership);
        let result = QoSPolicyId::from(
            cyclonedds_sys::dds_qos_policy_id_DDS_OWNERSHIPSTRENGTH_QOS_POLICY_ID,
        );
        assert_eq!(result, QoSPolicyId::OwnershipStrength);
        let result =
            QoSPolicyId::from(cyclonedds_sys::dds_qos_policy_id_DDS_LIVELINESS_QOS_POLICY_ID);
        assert_eq!(result, QoSPolicyId::Liveliness);
        let result =
            QoSPolicyId::from(cyclonedds_sys::dds_qos_policy_id_DDS_TIMEBASEDFILTER_QOS_POLICY_ID);
        assert_eq!(result, QoSPolicyId::TimeBasedFilter);
        let result =
            QoSPolicyId::from(cyclonedds_sys::dds_qos_policy_id_DDS_PARTITION_QOS_POLICY_ID);
        assert_eq!(result, QoSPolicyId::Partition);
        let result =
            QoSPolicyId::from(cyclonedds_sys::dds_qos_policy_id_DDS_RELIABILITY_QOS_POLICY_ID);
        assert_eq!(result, QoSPolicyId::Reliability);
        let result =
            QoSPolicyId::from(cyclonedds_sys::dds_qos_policy_id_DDS_DESTINATIONORDER_QOS_POLICY_ID);
        assert_eq!(result, QoSPolicyId::DestinationOrder);
        let result = QoSPolicyId::from(cyclonedds_sys::dds_qos_policy_id_DDS_HISTORY_QOS_POLICY_ID);
        assert_eq!(result, QoSPolicyId::History);
        let result =
            QoSPolicyId::from(cyclonedds_sys::dds_qos_policy_id_DDS_RESOURCELIMITS_QOS_POLICY_ID);
        assert_eq!(result, QoSPolicyId::ResourceLimits);
        let result =
            QoSPolicyId::from(cyclonedds_sys::dds_qos_policy_id_DDS_ENTITYFACTORY_QOS_POLICY_ID);
        assert_eq!(result, QoSPolicyId::EntityFactory);
        let result = QoSPolicyId::from(
            cyclonedds_sys::dds_qos_policy_id_DDS_WRITERDATALIFECYCLE_QOS_POLICY_ID,
        );
        assert_eq!(result, QoSPolicyId::WriterDataLifecycle);
        let result = QoSPolicyId::from(
            cyclonedds_sys::dds_qos_policy_id_DDS_READERDATALIFECYCLE_QOS_POLICY_ID,
        );
        assert_eq!(result, QoSPolicyId::ReaderDataLifecycle);
        let result =
            QoSPolicyId::from(cyclonedds_sys::dds_qos_policy_id_DDS_TOPICDATA_QOS_POLICY_ID);
        assert_eq!(result, QoSPolicyId::TopicData);
        let result =
            QoSPolicyId::from(cyclonedds_sys::dds_qos_policy_id_DDS_GROUPDATA_QOS_POLICY_ID);
        assert_eq!(result, QoSPolicyId::GroupData);
        let result = QoSPolicyId::from(
            cyclonedds_sys::dds_qos_policy_id_DDS_TRANSPORTPRIORITY_QOS_POLICY_ID,
        );
        assert_eq!(result, QoSPolicyId::TransportPriority);
        let result =
            QoSPolicyId::from(cyclonedds_sys::dds_qos_policy_id_DDS_LIFESPAN_QOS_POLICY_ID);
        assert_eq!(result, QoSPolicyId::Lifespan);
        let result = QoSPolicyId::from(
            cyclonedds_sys::dds_qos_policy_id_DDS_DURABILITYSERVICE_QOS_POLICY_ID,
        );
        assert_eq!(result, QoSPolicyId::DurabilityService);
        let result =
            QoSPolicyId::from(cyclonedds_sys::dds_qos_policy_id_DDS_PROPERTY_QOS_POLICY_ID);
        assert_eq!(result, QoSPolicyId::Property);
        let result = QoSPolicyId::from(
            cyclonedds_sys::dds_qos_policy_id_DDS_TYPE_CONSISTENCY_ENFORCEMENT_QOS_POLICY_ID,
        );
        assert_eq!(result, QoSPolicyId::TypeConsistencyEnforcement);
        let result = QoSPolicyId::from(
            cyclonedds_sys::dds_qos_policy_id_DDS_DATA_REPRESENTATION_QOS_POLICY_ID,
        );
        assert_eq!(result, QoSPolicyId::DataRepresentation);
    }

    #[test]
    #[should_panic]
    fn test_qos_policy_id_conversion_out_of_range() {
        let _ = QoSPolicyId::from(cyclonedds_sys::dds_qos_policy_id_t::MAX);
    }

    #[test]
    fn test_sample_rejected_reason_conversion() {
        let result = SampleRejectedReason::from(
            cyclonedds_sys::dds_sample_rejected_status_kind_DDS_NOT_REJECTED,
        );
        assert_eq!(result, SampleRejectedReason::NotRejected);
        let result = SampleRejectedReason::from(
            cyclonedds_sys::dds_sample_rejected_status_kind_DDS_REJECTED_BY_INSTANCES_LIMIT,
        );
        assert_eq!(result, SampleRejectedReason::RejectedByInstancesLimit);
        let result = SampleRejectedReason::from(
            cyclonedds_sys::dds_sample_rejected_status_kind_DDS_REJECTED_BY_SAMPLES_LIMIT,
        );
        assert_eq!(result, SampleRejectedReason::RejectedBySamplesLimit);
        let result = SampleRejectedReason::from(cyclonedds_sys::dds_sample_rejected_status_kind_DDS_REJECTED_BY_SAMPLES_PER_INSTANCE_LIMIT);
        assert_eq!(
            result,
            SampleRejectedReason::RejectedBySamplesPerInstanceLimit
        );
    }

    #[test]
    #[should_panic]
    fn test_sample_rejected_reason_conversion_out_of_range() {
        let _ = SampleRejectedReason::from(cyclonedds_sys::dds_sample_rejected_status_kind::MAX);
    }

    #[test]
    fn test_inconsistent_topic_conversion() {
        let total_count = 10;
        let total_count_change = 20;

        let status = InconsistentTopic::from(cyclonedds_sys::dds_inconsistent_topic_status {
            total_count,
            total_count_change,
        });

        assert_eq!(
            (status.total.value, status.total.delta),
            (total_count, total_count_change)
        );
    }

    #[test]
    fn test_liveliness_lost_conversion() {
        let total_count = 10;
        let total_count_change = 20;
        let status = LivelinessLost::from(cyclonedds_sys::dds_liveliness_lost_status {
            total_count,
            total_count_change,
        });

        assert_eq!(
            (status.total.value, status.total.delta),
            (total_count, total_count_change)
        );
    }

    #[test]
    fn test_offered_deadline_missed_conversion() {
        let total_count = 10;
        let total_count_change = 20;
        let last_instance_handle = 30;

        let status =
            OfferedDeadlineMissed::from(cyclonedds_sys::dds_offered_deadline_missed_status {
                total_count,
                total_count_change,
                last_instance_handle,
            });

        assert_eq!(
            (
                status.total.value,
                status.total.delta,
                status.last_instance_handle.inner
            ),
            (total_count, total_count_change, last_instance_handle)
        );
    }

    #[test]
    fn test_offered_incompatible_qos_conversion() {
        let total_count = 10;
        let total_count_change = 20;
        let last_policy_id = cyclonedds_sys::dds_qos_policy_id_DDS_DURABILITY_QOS_POLICY_ID;

        let status =
            OfferedIncompatibleQoS::from(cyclonedds_sys::dds_offered_incompatible_qos_status {
                total_count,
                total_count_change,
                last_policy_id,
            });

        assert_eq!(
            (
                status.total.value,
                status.total.delta,
                status.last_policy_id
            ),
            (total_count, total_count_change, last_policy_id.into())
        )
    }

    #[test]
    fn test_publication_matched_conversion() {
        let total_count = 10;
        let total_count_change = 20;
        let current_count = 30;
        let current_count_change = 40;
        let last_subscription_handle = 50;

        let status = PublicationMatched::from(cyclonedds_sys::dds_publication_matched_status {
            total_count,
            total_count_change,
            current_count,
            current_count_change,
            last_subscription_handle,
        });

        assert_eq!(
            (
                status.total.value,
                status.total.delta,
                status.current.value,
                status.current.delta,
                status.last_subscription_handle.inner
            ),
            (
                total_count,
                total_count_change,
                current_count,
                current_count_change,
                last_subscription_handle
            )
        )
    }

    #[test]
    fn test_sample_lost_conversion() {
        let total_count = 10;
        let total_count_change = 20;

        let status = SampleLost::from(cyclonedds_sys::dds_sample_lost_status {
            total_count,
            total_count_change,
        });

        assert_eq!(
            (status.total.value, status.total.delta),
            (total_count, total_count_change)
        );
    }

    #[test]
    fn test_sample_rejected_conversion() {
        let total_count = 10;
        let total_count_change = 20;
        let last_reason =
            cyclonedds_sys::dds_sample_rejected_status_kind_DDS_REJECTED_BY_INSTANCES_LIMIT;
        let last_instance_handle = 40;

        let status = SampleRejected::from(cyclonedds_sys::dds_sample_rejected_status {
            total_count,
            total_count_change,
            last_reason,
            last_instance_handle,
        });

        assert_eq!(
            (
                status.total.value,
                status.total.delta,
                status.last_reason,
                status.last_instance_handle.inner
            ),
            (
                total_count,
                total_count_change,
                last_reason.into(),
                last_instance_handle
            )
        )
    }

    #[test]
    fn test_liveliness_changed_conversion() {
        let alive_count = 10;
        let alive_count_change = 20;
        let not_alive_count = 30;
        let not_alive_count_change = 40;
        let last_publication_handle = 50;

        let status = LivelinessChanged::from(cyclonedds_sys::dds_liveliness_changed_status {
            alive_count,
            alive_count_change,
            not_alive_count,
            not_alive_count_change,
            last_publication_handle,
        });

        assert_eq!(
            (
                status.alive.value,
                status.alive.delta,
                status.not_alive.value,
                status.not_alive.delta,
                status.last_publication_handle.inner
            ),
            (
                alive_count,
                alive_count_change,
                not_alive_count,
                not_alive_count_change,
                last_publication_handle
            )
        )
    }

    #[test]
    fn test_requested_deadline_missed_conversion() {
        let total_count = 10;
        let total_count_change = 20;
        let last_instance_handle = 30;

        let status =
            RequestedDeadlineMissed::from(cyclonedds_sys::dds_requested_deadline_missed_status {
                total_count,
                total_count_change,
                last_instance_handle,
            });

        assert_eq!(
            (
                status.total.value,
                status.total.delta,
                status.last_instance_handle.inner
            ),
            (total_count, total_count_change, last_instance_handle)
        )
    }

    #[test]
    fn test_requested_incompatible_qos_conversion() {
        let total_count = 10;
        let total_count_change = 20;
        let last_policy_id = cyclonedds_sys::dds_qos_policy_id_DDS_DURABILITY_QOS_POLICY_ID;

        let status =
            RequestedIncompatibleQoS::from(cyclonedds_sys::dds_requested_incompatible_qos_status {
                total_count,
                total_count_change,
                last_policy_id,
            });

        assert_eq!(
            (
                status.total.value,
                status.total.delta,
                status.last_policy_id
            ),
            (total_count, total_count_change, last_policy_id.into())
        );
    }

    #[test]
    fn test_subscription_matched_conversion() {
        let total_count = 10;
        let total_count_change = 20;
        let current_count = 30;
        let current_count_change = 40;
        let last_publication_handle = 50;

        let status = SubscriptionMatched::from(cyclonedds_sys::dds_subscription_matched_status {
            total_count,
            total_count_change,
            current_count,
            current_count_change,
            last_publication_handle,
        });

        assert_eq!(
            (
                status.total.value,
                status.total.delta,
                status.current.value,
                status.current.delta,
                status.last_publication_handle.inner
            ),
            (
                total_count,
                total_count_change,
                current_count,
                current_count_change,
                last_publication_handle
            )
        )
    }
}
