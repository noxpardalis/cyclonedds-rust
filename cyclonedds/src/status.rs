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
