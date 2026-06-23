//! Readers and sample types for DDS built-in topics.
//!
//! Built-in topics expose discovery data that DDS maintains for participants,
//! topics, publications, and subscriptions. The concrete reader
//! aliases are the primary entry points:
//!
//! - [`DcpsParticipantReader`] reads [`DcpsParticipant`] samples.
//! - [`DcpsPublicationReader`] reads [`DcpsPublication`] samples.
//! - [`DcpsSubscriptionReader`] reads [`DcpsSubscription`] samples.
//! - [`DcpsTopicReader`] reads [`DcpsTopic`] samples.
//!
//! All built-in topic readers return
//! [`SampleOrKey`](crate::sample::SampleOrKey) values, just like regular
//! [`Readers`](crate::Reader). A key-only sample is materialized with
//! [`Sample::from_key`](crate::sample::Sample::from_key) using the built-in
//! sample type's associated [`Key`](crate::sample::Sample::Key) type.
//!
//! # Examples
//!
//! ```no_run
//! use cyclonedds::{Domain, Participant, builtin, sample::View};
//!
//! let domain = Domain::default();
//! let participant = Participant::new(&domain)?;
//! let reader = builtin::DcpsPublicationReader::new(&participant)?;
//!
//! for sample in reader.take()? {
//!     match sample.view() {
//!         View::Sample(publication) => {
//!             println!(
//!                 "writer on {} with type {}",
//!                 publication.endpoint.topic_name,
//!                 publication.endpoint.type_name,
//!             );
//!         }
//!         View::Key(key) => {
//!             println!("writer disposed or unregistered: {key:?}");
//!         }
//!     }
//! }
//! # Ok::<_, cyclonedds::Error>(())
//! ```

use crate::internal::ffi;
use crate::{Participant, QoS, Result};
use private::BuiltInTopicReaderBuilder;

/// GUID key used by built-in topic samples.
///
/// This is the [`Key`](crate::sample::Sample::Key) type for
/// [`DcpsParticipant`] and [`DcpsTopic`] samples. For [`DcpsParticipant`], it
/// is the GUID of the discovered participant. For [`DcpsTopic`], it is the
/// GUID assigned to the discovered topic.
///
/// Endpoint built-in topic samples use [`DcpsEndpointKey`] as their
/// [`Key`](crate::sample::Sample::Key), which contains `BuiltInTopicKey`
/// fields for the endpoint and its owning participant.
pub type BuiltInTopicKey = crate::entity::Guid;

/// Key fields shared by publication and subscription built-in topic samples.
///
/// This is the [`Key`](crate::sample::Sample::Key) type for
/// [`DcpsPublication`] and [`DcpsSubscription`] samples. Key-only publication
/// and subscription notifications materialize [`DcpsEndpoint`] with these
/// fields filled in and the remaining endpoint fields left at their defaults.
#[derive(Debug, Clone, Copy, Default)]
pub struct DcpsEndpointKey {
    /// GUID of the discovered reader or writer.
    pub key: BuiltInTopicKey,
    /// GUID of the participant that owns the endpoint.
    pub participant_key: BuiltInTopicKey,
    /// Instance handle assigned to the participant that owns the endpoint.
    pub participant_instance_handle: crate::entity::InstanceHandle,
}

/// Participant discovery data from the `DCPSParticipant` built-in topic.
///
/// The [`Key`](crate::sample::Sample::Key) type is [`BuiltInTopicKey`]. A
/// participant key is the GUID of the discovered participant. A key-only
/// notification materializes this type with [`key`](DcpsParticipant::key) set
/// to the received key and all other fields left at their defaults.
///
/// Prefer [`DcpsParticipantReader`] when creating a reader for this topic.
#[derive(Debug, Clone, Default)]
pub struct DcpsParticipant {
    /// GUID of the discovered participant.
    pub key: BuiltInTopicKey,
    /// `QoS` policies associated with the endpoint.
    pub qos: QoS,
}

/// Topic discovery data from the `DCPSTopic` built-in topic.
///
/// Cyclone DDS only publishes this topic when topic discovery is enabled.
///
/// The [`Key`](crate::sample::Sample::Key) type is [`BuiltInTopicKey`]. A
/// topic key is the GUID assigned to the discovered topic. A key-only
/// notification materializes this type with [`key`](DcpsTopic::key) set to the
/// received key and all other fields left at their defaults.
///
/// Prefer [`DcpsTopicReader`] when creating a reader for this topic.
#[derive(Debug, Default)]
pub struct DcpsTopic {
    /// Key that uniquely identifies the discovered topic.
    pub key: BuiltInTopicKey,
    /// Topic name.
    pub topic_name: String,
    /// Type name.
    pub type_name: String,
    /// `QoS` policies associated with the endpoint.
    pub qos: QoS,
}

/// Endpoint discovery data shared by publication and subscription built-in
/// topics.
#[derive(Debug, Default)]
pub struct DcpsEndpoint {
    /// GUID of the discovered reader or writer.
    pub key: BuiltInTopicKey,
    /// GUID of the participant that owns this endpoint.
    pub participant_key: BuiltInTopicKey,
    /// Instance handle assigned to the participant that owns this endpoint.
    pub participant_instance_handle: crate::entity::InstanceHandle,
    /// Topic name used by the endpoint.
    pub topic_name: String,
    /// Type name used by the endpoint.
    pub type_name: String,
    /// `QoS` policies associated with the endpoint.
    pub qos: QoS,
}

/// Publication discovery data from the `DCPSPublication` built-in topic.
///
/// The [`Key`](crate::sample::Sample::Key) type is [`DcpsEndpointKey`]. A
/// key-only notification materializes [`endpoint`](DcpsPublication::endpoint)
/// with the key, participant key, and participant instance handle filled in and
/// all other endpoint fields left at their defaults.
///
/// Prefer [`DcpsPublicationReader`] when creating a reader for this topic.
#[derive(Debug, Default)]
pub struct DcpsPublication {
    /// Endpoint data for the discovered writer.
    pub endpoint: DcpsEndpoint,
}

/// Subscription discovery data from the `DCPSSubscription` built-in topic.
///
/// The [`Key`](crate::sample::Sample::Key) type is [`DcpsEndpointKey`]. A
/// key-only notification materializes [`endpoint`](DcpsSubscription::endpoint)
/// with the key, participant key, and participant instance handle filled in and
/// all other endpoint fields left at their defaults.
///
/// Prefer [`DcpsSubscriptionReader`] when creating a reader for this topic.
#[derive(Debug, Default)]
pub struct DcpsSubscription {
    /// Endpoint data for the discovered reader.
    pub endpoint: DcpsEndpoint,
}

pub(crate) mod private {
    use super::BuiltInTopicReader;
    use crate::internal::ffi;
    use crate::internal::ffi::FromFfi;
    use crate::internal::traits::AsFfi;
    use crate::{Participant, QoS, Result, Subscriber};

    /// Type marker for built-in topic samples.
    pub trait BuiltInTopicType:
        std::fmt::Debug + FromFfi<Source = Self::Type> + crate::sample::Sample
    {
        type Type: std::default::Default + Clone + Copy;

        const TOPIC: cyclonedds_sys::dds_entity_t;
    }

    impl crate::sample::sealed::Sealed for super::DcpsParticipant {}

    impl crate::sample::Sample for super::DcpsParticipant {
        type Key = super::BuiltInTopicKey;

        fn from_key(key: &Self::Key) -> Self {
            Self {
                key: *key,
                ..Self::default()
            }
        }

        fn as_key(&self) -> Self::Key {
            self.key
        }
    }
    impl BuiltInTopicType for super::DcpsParticipant {
        type Type = cyclonedds_sys::dds_builtintopic_participant_t;

        const TOPIC: cyclonedds_sys::dds_entity_t = cyclonedds_sys::BUILTIN_TOPIC_DCPS_PARTICIPANT;
    }

    impl crate::sample::sealed::Sealed for super::DcpsTopic {}

    impl crate::sample::Sample for super::DcpsTopic {
        type Key = super::BuiltInTopicKey;

        fn from_key(key: &Self::Key) -> Self {
            Self {
                key: *key,
                ..Self::default()
            }
        }

        fn as_key(&self) -> Self::Key {
            self.key
        }
    }
    impl BuiltInTopicType for super::DcpsTopic {
        type Type = cyclonedds_sys::dds_builtintopic_topic_t;

        const TOPIC: cyclonedds_sys::dds_entity_t = cyclonedds_sys::BUILTIN_TOPIC_DCPS_TOPIC;
    }

    impl crate::sample::sealed::Sealed for super::DcpsPublication {}

    impl crate::sample::Sample for super::DcpsPublication {
        type Key = super::DcpsEndpointKey;

        fn from_key(endpoint: &Self::Key) -> Self {
            Self {
                endpoint: super::DcpsEndpoint {
                    key: endpoint.key,
                    participant_key: endpoint.participant_key,
                    participant_instance_handle: endpoint.participant_instance_handle,
                    ..super::DcpsEndpoint::default()
                },
            }
        }

        fn as_key(&self) -> Self::Key {
            super::DcpsEndpointKey {
                key: self.endpoint.key,
                participant_key: self.endpoint.participant_key,
                participant_instance_handle: self.endpoint.participant_instance_handle,
            }
        }
    }
    impl BuiltInTopicType for super::DcpsPublication {
        type Type = cyclonedds_sys::dds_builtintopic_endpoint_t;
        const TOPIC: cyclonedds_sys::dds_entity_t = cyclonedds_sys::BUILTIN_TOPIC_DCPS_PUBLICATION;
    }

    impl crate::sample::sealed::Sealed for super::DcpsSubscription {}

    impl crate::sample::Sample for super::DcpsSubscription {
        type Key = super::DcpsEndpointKey;

        fn from_key(endpoint: &Self::Key) -> Self {
            Self {
                endpoint: super::DcpsEndpoint {
                    key: endpoint.key,
                    participant_key: endpoint.participant_key,
                    participant_instance_handle: endpoint.participant_instance_handle,
                    ..super::DcpsEndpoint::default()
                },
            }
        }

        fn as_key(&self) -> Self::Key {
            super::DcpsEndpointKey {
                key: self.endpoint.key,
                participant_key: self.endpoint.participant_key,
                participant_instance_handle: self.endpoint.participant_instance_handle,
            }
        }
    }
    impl BuiltInTopicType for super::DcpsSubscription {
        type Type = cyclonedds_sys::dds_builtintopic_endpoint_t;
        const TOPIC: cyclonedds_sys::dds_entity_t = cyclonedds_sys::BUILTIN_TOPIC_DCPS_SUBSCRIPTION;
    }

    /// Builder for [`BuiltInTopicReader`] (accessible via [`BuiltInTopicReader::builder`]).
    #[derive(Debug)]
    pub struct BuiltInTopicReaderBuilder<'domain, 'participant, 'qos, T>
    where
        T: BuiltInTopicType,
    {
        participant: &'participant Participant<'domain>,
        subscriber: Option<&'participant Subscriber<'domain, 'participant>>,
        qos: Option<&'qos QoS>,
        phantom_data: std::marker::PhantomData<T>,
    }

    impl<'d, 'p, 'q, T> BuiltInTopicReaderBuilder<'d, 'p, 'q, T>
    where
        T: BuiltInTopicType,
    {
        /// Creates a new [`BuiltInTopicReaderBuilder`] for the given
        /// [`Participant`].
        ///
        /// # Examples
        ///
        /// ```no_run
        /// use cyclonedds::builtin::{BuiltInTopicReaderBuilder, DcpsParticipant};
        /// use cyclonedds::{Domain, Participant};
        ///
        /// let domain = Domain::default();
        /// let participant = Participant::new(&domain)?;
        /// let reader_builder = BuiltInTopicReaderBuilder::<DcpsParticipant>::new(&participant);
        /// # let _ = reader_builder;
        /// # Ok::<_, cyclonedds::Error>(())
        /// ```
        #[must_use]
        pub const fn new(participant: &'p Participant<'d>) -> Self {
            Self {
                participant,
                subscriber: None,
                qos: None,
                phantom_data: std::marker::PhantomData,
            }
        }

        /// Sets the [`Subscriber`] on this built-in topic reader builder.
        ///
        /// # Examples
        ///
        /// ```no_run
        /// use cyclonedds::builtin::DcpsParticipantReader;
        /// use cyclonedds::{Domain, Participant, Subscriber};
        ///
        /// let domain = Domain::default();
        /// let participant = Participant::new(&domain)?;
        /// let subscriber = Subscriber::new(&participant)?;
        ///
        /// let reader_builder = DcpsParticipantReader::builder(&participant)
        ///     .with_subscriber(&subscriber);
        /// # let _ = reader_builder;
        /// # Ok::<_, cyclonedds::Error>(())
        /// ```
        #[must_use]
        pub const fn with_subscriber(mut self, subscriber: &'p Subscriber<'d, 'p>) -> Self {
            self.subscriber = Some(subscriber);
            self
        }

        /// Sets the [`QoS`] for this built-in topic reader builder.
        ///
        /// # Examples
        ///
        /// ```no_run
        /// use cyclonedds::builtin::DcpsParticipantReader;
        /// use cyclonedds::qos::policy;
        /// use cyclonedds::{Domain, Participant, QoS};
        ///
        /// let domain = Domain::default();
        /// let participant = Participant::new(&domain)?;
        /// let qos = QoS::new().with_reliability(policy::Reliability::BestEffort);
        ///
        /// let reader_builder = DcpsParticipantReader::builder(&participant).with_qos(&qos);
        /// # let _ = reader_builder;
        /// # Ok::<_, cyclonedds::Error>(())
        /// ```
        #[must_use]
        pub const fn with_qos(mut self, qos: &'q QoS) -> Self {
            self.qos = Some(qos);
            self
        }

        /// Builds the [`BuiltInTopicReader`].
        ///
        /// # Errors
        ///
        /// Returns an [`Error`](crate::Error) if the reader failed to create. In
        /// particular, `DCPSTopic` returns [`Error::Unsupported`](crate::Error) if
        /// topic discovery is not enabled in the C build.
        ///
        /// # Examples
        ///
        /// ```no_run
        /// use cyclonedds::builtin::DcpsPublicationReader;
        /// use cyclonedds::{Domain, Participant};
        ///
        /// let domain = Domain::default();
        /// let participant = Participant::new(&domain)?;
        /// let reader = DcpsPublicationReader::builder(&participant).build()?;
        /// # let _ = reader;
        /// # Ok::<_, cyclonedds::Error>(())
        /// ```
        pub fn build(self) -> Result<BuiltInTopicReader<'d, 'p, T>> {
            let parent = self
                .subscriber
                .map_or(self.participant.inner, |subscriber| subscriber.inner);
            let qos = self.qos.map(crate::QoS::as_ffi);
            let inner = ffi::dds_create_reader(parent, T::TOPIC, qos.as_ref(), None)?;

            Ok(BuiltInTopicReader {
                inner,
                phantom_participant: std::marker::PhantomData,
                phantom_data: std::marker::PhantomData,
            })
        }
    }
}

/// A built-in topic reader for [`DcpsParticipant`] samples.
///
/// Use this alias as the entry point for reading discovered participants.
///
/// # Examples
///
/// ```no_run
/// use cyclonedds::{Domain, Participant, builtin};
///
/// let domain = Domain::default();
/// let participant = Participant::new(&domain)?;
/// let reader = builtin::DcpsParticipantReader::new(&participant)?;
///
/// for sample in reader.take()? {
///     if let Some(participant) = sample.sample() {
///         println!("participant: {:?}", participant.key);
///     }
/// }
/// # Ok::<_, cyclonedds::Error>(())
/// ```
pub type DcpsParticipantReader<'domain, 'participant> =
    BuiltInTopicReader<'domain, 'participant, DcpsParticipant>;

/// A built-in topic reader for [`DcpsTopic`] samples.
///
/// Use this alias as the entry point for reading discovered topics.
///
/// Cyclone DDS only supports this reader when topic discovery is enabled.
///
/// # Examples
///
/// ```no_run
/// use cyclonedds::{Domain, Participant, builtin};
///
/// let domain = Domain::new_with_xml_config(
///     0,
///     "<CycloneDDS><Domain><Discovery><EnableTopicDiscoveryEndpoints>true</EnableTopicDiscoveryEndpoints></Discovery></Domain></CycloneDDS>",
/// )?;
/// let participant = Participant::new(&domain)?;
/// let reader = builtin::DcpsTopicReader::new(&participant)?;
///
/// for sample in reader.read()? {
///     if let Some(topic) = sample.sample() {
///         println!("topic {} has type {}", topic.topic_name, topic.type_name);
///     }
/// }
/// # Ok::<_, cyclonedds::Error>(())
/// ```
pub type DcpsTopicReader<'domain, 'participant> =
    BuiltInTopicReader<'domain, 'participant, DcpsTopic>;

/// A built-in topic reader for [`DcpsPublication`] samples.
///
/// Use this alias as the entry point for reading discovered writers.
///
/// # Examples
///
/// ```no_run
/// use cyclonedds::{Domain, Participant, builtin};
///
/// let domain = Domain::default();
/// let participant = Participant::new(&domain)?;
/// let reader = builtin::DcpsPublicationReader::new(&participant)?;
///
/// for sample in reader.take()? {
///     if let Some(publication) = sample.sample() {
///         println!("publication on {}", publication.endpoint.topic_name);
///     }
/// }
/// # Ok::<_, cyclonedds::Error>(())
/// ```
pub type DcpsPublicationReader<'domain, 'participant> =
    BuiltInTopicReader<'domain, 'participant, DcpsPublication>;

/// A built-in topic reader for [`DcpsSubscription`] samples.
///
/// Use this alias as the entry point for reading discovered readers.
///
/// # Examples
///
/// ```no_run
/// use cyclonedds::{Domain, Participant, builtin};
///
/// let domain = Domain::default();
/// let participant = Participant::new(&domain)?;
/// let reader = builtin::DcpsSubscriptionReader::new(&participant)?;
///
/// for sample in reader.take()? {
///     if let Some(subscription) = sample.sample() {
///         println!("subscription on {}", subscription.endpoint.topic_name);
///     }
/// }
/// # Ok::<_, cyclonedds::Error>(())
/// ```
pub type DcpsSubscriptionReader<'domain, 'participant> =
    BuiltInTopicReader<'domain, 'participant, DcpsSubscription>;

/// A reader for built-in topic type `T`.
///
/// A `BuiltInTopicReader` receives discovery samples for one of the DDS
/// built-in topics. Samples are retrieved via
/// [`read`](BuiltInTopicReader::read), [`take`](BuiltInTopicReader::take), or
/// [`peek`](BuiltInTopicReader::peek).
///
/// Use the concrete reader aliases in application code:
///
/// - [`DcpsParticipantReader`]
/// - [`DcpsPublicationReader`]
/// - [`DcpsSubscriptionReader`]
/// - [`DcpsTopicReader`]
///
/// Use [`BuiltInTopicReader`] directly when writing code that is generic over
/// built-in sample types.
#[derive(Debug, PartialEq, Eq)]
pub struct BuiltInTopicReader<'domain, 'participant, T>
where
    T: private::BuiltInTopicType,
{
    pub(crate) inner: cyclonedds_sys::dds_entity_t,
    phantom_participant: std::marker::PhantomData<&'participant crate::Participant<'domain>>,
    phantom_data: std::marker::PhantomData<T>,
}

impl<'d, 'p, T> BuiltInTopicReader<'d, 'p, T>
where
    T: private::BuiltInTopicType,
{
    /// Creates a built-in topic reader with default [`QoS`].
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if the reader fails to create.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cyclonedds::{Domain, Participant, builtin};
    ///
    /// let domain = Domain::default();
    /// let participant = Participant::new(&domain)?;
    /// let reader = builtin::DcpsPublicationReader::new(&participant)?;
    /// # let _ = reader;
    /// # Ok::<_, cyclonedds::Error>(())
    /// ```
    pub fn new(participant: &'p Participant<'d>) -> Result<Self> {
        Self::builder(participant).build()
    }

    /// Returns a [`BuiltInTopicReaderBuilder`] for constructing a reader with
    /// custom [`QoS`] or a specific [`Subscriber`](crate::Subscriber).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cyclonedds::{Domain, Participant, QoS, Subscriber, builtin, qos};
    ///
    /// let domain = Domain::default();
    /// let participant = Participant::new(&domain)?;
    /// let subscriber = Subscriber::new(&participant)?;
    /// let qos = QoS::new().with_reliability(qos::policy::Reliability::BestEffort);
    ///
    /// let reader = builtin::DcpsSubscriptionReader::builder(&participant)
    ///     .with_subscriber(&subscriber)
    ///     .with_qos(&qos)
    ///     .build()?;
    /// # let _ = reader;
    /// # Ok::<_, cyclonedds::Error>(())
    /// ```
    #[must_use]
    pub const fn builder<'q>(
        participant: &'p Participant<'d>,
    ) -> BuiltInTopicReaderBuilder<'d, 'p, 'q, T> {
        BuiltInTopicReaderBuilder::new(participant)
    }

    /// Removes and returns all available samples from the reader cache.
    ///
    /// Each call to `take` consumes the returned samples so they will not be
    /// returned by subsequent calls. See [`read`](BuiltInTopicReader::read) to
    /// leave samples in the cache.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if the reader fails to take samples.
    pub fn take(&self) -> Result<Vec<crate::sample::SampleOrKey<T>>> {
        ffi::builtin::dds_take(self.inner)
    }

    /// Returns all available samples from the reader cache without removing
    /// them.
    ///
    /// Samples returned by `read` remain in the cache and will be returned
    /// again by subsequent calls. See [`take`](BuiltInTopicReader::take) to
    /// consume samples.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if the reader fails to read samples.
    pub fn read(&self) -> Result<Vec<crate::sample::SampleOrKey<T>>> {
        ffi::builtin::dds_read(self.inner)
    }

    /// Returns all available samples without marking them as read or removing
    /// them from the cache.
    ///
    /// Useful for checking whether data is available without affecting the
    /// read state of samples. Subsequent calls to
    /// [`read`](BuiltInTopicReader::read) or [`take`](BuiltInTopicReader::take)
    /// will still return the same samples as unread.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if the reader fails to peek.
    pub fn peek(&self) -> Result<Vec<crate::sample::SampleOrKey<T>>> {
        ffi::builtin::dds_peek(self.inner)
    }
}

impl<T> Drop for BuiltInTopicReader<'_, '_, T>
where
    T: private::BuiltInTopicType,
{
    fn drop(&mut self) {
        let result = ffi::dds_delete(self.inner);
        debug_assert!(
            result.is_ok(),
            "unable to delete {self:?}: failed with {result:?}"
        );
    }
}
