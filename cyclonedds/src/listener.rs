//!
use crate::Result;
use crate::internal::ffi;
use crate::status::{
    InconsistentTopic, LivelinessChanged, LivelinessLost, OfferedDeadlineMissed,
    OfferedIncompatibleQoS, PublicationMatched, RequestedDeadlineMissed, RequestedIncompatibleQoS,
    SampleLost, SampleRejected, SubscriptionMatched,
};

///
#[derive(Debug, Default)]
pub struct Listener {
    // topic: TopicListener<T>,
    subscriber: SubscriberListener,
    publisher: PublisherListener,
}

///
#[derive(Debug)]
pub struct TopicListener<T>
where
    T: crate::Topicable,
{
    inconsistent_topic: Option<fn(&crate::Topic<T>, InconsistentTopic)>,
}

///
#[derive(Debug, Default)]
pub struct SubscriberListener {
    data_on_readers: Option<fn(&crate::Subscriber)>,
    // ///
    // pub reader: ReaderListener<T>,
}

///
#[derive(Debug)]
pub struct ReaderListener<T>
where
    T: crate::Topicable,
{
    sample_lost: Option<fn(&crate::Reader<T>, SampleLost)>,
    data_available: Option<fn(&crate::Reader<T>)>,
    sample_rejected: Option<fn(&crate::Reader<T>, SampleRejected)>,
    liveliness_changed: Option<fn(&crate::Reader<T>, LivelinessChanged)>,
    requested_deadline_missed: Option<fn(&crate::Reader<T>, RequestedDeadlineMissed)>,
    requested_incompatible_qos: Option<fn(&crate::Reader<T>, RequestedIncompatibleQoS)>,
    subscription_matched: Option<fn(&crate::Reader<T>, SubscriptionMatched)>,
}

///
#[derive(Debug, Default)]
pub struct PublisherListener {
    // ///
    // pub writer: WriterListener<T>,
}

///
#[derive(Debug)]
pub struct WriterListener<T>
where
    T: crate::Topicable,
{
    liveliness_lost: Option<fn(&crate::Writer<T>, LivelinessLost)>,
    offered_deadline_missed: Option<fn(&crate::Writer<T>, OfferedDeadlineMissed)>,
    offered_incompatible_qos: Option<fn(&crate::Writer<T>, OfferedIncompatibleQoS)>,
    publication_matched: Option<fn(&crate::Writer<T>, PublicationMatched)>,
}

impl<T> Default for TopicListener<T>
where
    T: crate::Topicable,
{
    fn default() -> Self {
        Self {
            inconsistent_topic: Default::default(),
        }
    }
}

impl<T> Default for ReaderListener<T>
where
    T: crate::Topicable,
{
    fn default() -> Self {
        Self {
            sample_lost: Default::default(),
            data_available: Default::default(),
            sample_rejected: Default::default(),
            liveliness_changed: Default::default(),
            requested_deadline_missed: Default::default(),
            requested_incompatible_qos: Default::default(),
            subscription_matched: Default::default(),
        }
    }
}

impl<T> Default for WriterListener<T>
where
    T: crate::Topicable,
{
    fn default() -> Self {
        Self {
            liveliness_lost: Default::default(),
            offered_deadline_missed: Default::default(),
            offered_incompatible_qos: Default::default(),
            publication_matched: Default::default(),
        }
    }
}

impl Listener {
    ///
    pub fn new() -> Self {
        Default::default()
    }

    // ///
    // pub fn with_topic(mut self, setter: fn(TopicListener<T>) -> TopicListener<T>) -> Self {
    //     self.topic = setter(self.topic);
    //     self
    // }

    ///
    pub fn with_subscriber(mut self, setter: fn(SubscriberListener) -> SubscriberListener) -> Self {
        self.subscriber = setter(self.subscriber);
        self
    }

    ///
    pub fn with_publisher(mut self, setter: fn(PublisherListener) -> PublisherListener) -> Self {
        self.publisher = setter(self.publisher);
        self
    }

    ///
    #[inline]
    pub(crate) fn as_ffi(&self) -> Result<ffi::Listener> {
        ffi::Listener::new().map(|mut listener| {
            self.apply_listener_ffi(&mut listener);
            listener
        })
    }

    ///
    #[inline]
    pub(crate) fn apply_listener_ffi(&self, listener: &mut ffi::Listener) {
        // self.topic.apply_listener_ffi(listener);
        self.subscriber.apply_listener_ffi(listener);
        self.publisher.apply_listener_ffi(listener);
    }
}

impl<T> TopicListener<T>
where
    T: crate::Topicable,
{
    ///
    pub fn new() -> Self {
        Default::default()
    }

    ///
    pub fn with_inconsistent_topic(
        mut self,
        callback: fn(&crate::Topic<T>, InconsistentTopic),
    ) -> Self {
        self.inconsistent_topic = Some(callback);
        self
    }

    ///
    #[inline]
    pub(crate) fn as_ffi(&self) -> Result<ffi::Listener> {
        ffi::Listener::new().map(|mut listener| {
            self.apply_listener_ffi(&mut listener);
            listener
        })
    }

    ///
    #[inline]
    pub(crate) fn apply_listener_ffi(&self, listener: &mut ffi::Listener) {
        if let Some(callback) = self.inconsistent_topic {
            ffi::dds_listener_set_inconsistent_topic(listener, callback);
        }
    }
}

impl SubscriberListener {
    ///
    pub fn new() -> Self {
        Default::default()
    }

    // ///
    // pub fn with_reader(mut self, setter: fn(ReaderListener<T>) -> ReaderListener<T>) -> Self {
    //     self.reader = setter(self.reader);
    //     self
    // }

    ///
    pub fn with_data_on_readers(mut self, callback: fn(&crate::Subscriber)) -> Self {
        self.data_on_readers = Some(callback);
        self
    }

    ///
    #[inline]
    pub(crate) fn as_ffi(&self) -> Result<ffi::Listener> {
        ffi::Listener::new().map(|mut listener| {
            self.apply_listener_ffi(&mut listener);
            listener
        })
    }

    ///
    #[inline]
    pub(crate) fn apply_listener_ffi(&self, listener: &mut ffi::Listener) {
        if let Some(callback) = self.data_on_readers {
            ffi::dds_listener_set_data_on_readers(listener, callback);
        }
        // self.reader.apply_listener_ffi(listener);
    }
}

impl PublisherListener {
    ///
    pub fn new() -> Self {
        Default::default()
    }

    // ///
    // pub fn with_writer(mut self, setter: fn(WriterListener<T>) -> WriterListener<T>) -> Self {
    //     self.writer = setter(self.writer);
    //     self
    // }

    ///
    #[inline]
    pub(crate) fn as_ffi(&self) -> Result<ffi::Listener> {
        ffi::Listener::new().map(|mut listener| {
            self.apply_listener_ffi(&mut listener);
            listener
        })
    }

    #[inline]
    pub(crate) fn apply_listener_ffi(&self, listener: &mut ffi::Listener) {
        let _ = listener;
        // self.writer.apply_listener_ffi(listener);
    }
}

impl<T> ReaderListener<T>
where
    T: crate::Topicable,
{
    ///
    pub fn new() -> Self {
        Default::default()
    }

    ///
    pub fn with_sample_lost(mut self, callback: fn(&crate::Reader<T>, SampleLost)) -> Self {
        self.sample_lost = Some(callback);
        self
    }

    ///
    pub fn with_data_available(mut self, callback: fn(&crate::Reader<T>)) -> Self {
        self.data_available = Some(callback);
        self
    }

    ///
    pub fn with_sample_rejected(mut self, callback: fn(&crate::Reader<T>, SampleRejected)) -> Self {
        self.sample_rejected = Some(callback);
        self
    }

    ///
    pub fn with_liveliness_changed(
        mut self,
        callback: fn(&crate::Reader<T>, LivelinessChanged),
    ) -> Self {
        self.liveliness_changed = Some(callback);
        self
    }

    ///
    pub fn with_requested_deadline_missed(
        mut self,
        callback: fn(&crate::Reader<T>, RequestedDeadlineMissed),
    ) -> Self {
        self.requested_deadline_missed = Some(callback);
        self
    }

    ///
    pub fn with_requested_incompatible_qos(
        mut self,
        callback: fn(&crate::Reader<T>, RequestedIncompatibleQoS),
    ) -> Self {
        self.requested_incompatible_qos = Some(callback);
        self
    }

    ///
    pub fn with_subscription_matched(
        mut self,
        callback: fn(&crate::Reader<T>, SubscriptionMatched),
    ) -> Self {
        self.subscription_matched = Some(callback);
        self
    }

    ///
    #[inline]
    pub(crate) fn as_ffi(&self) -> Result<ffi::Listener> {
        ffi::Listener::new().map(|mut listener| {
            self.apply_listener_ffi(&mut listener);
            listener
        })
    }

    ///
    #[inline]
    pub(crate) fn apply_listener_ffi(&self, listener: &mut ffi::Listener) {
        if let Some(callback) = self.sample_lost {
            ffi::dds_listener_set_sample_lost(listener, callback);
        }
        if let Some(callback) = self.data_available {
            ffi::dds_listener_set_data_available(listener, callback);
        }
        if let Some(callback) = self.sample_rejected {
            ffi::dds_listener_set_sample_rejected(listener, callback);
        }
        if let Some(callback) = self.liveliness_changed {
            ffi::dds_listener_set_liveliness_changed(listener, callback);
        }
        if let Some(callback) = self.requested_deadline_missed {
            ffi::dds_listener_set_requested_deadline_missed(listener, callback);
        }
        if let Some(callback) = self.requested_incompatible_qos {
            ffi::dds_listener_set_requested_incompatible_qos(listener, callback);
        }
        if let Some(callback) = self.subscription_matched {
            ffi::dds_listener_set_subscription_matched(listener, callback);
        }
    }
}

impl<T> WriterListener<T>
where
    T: crate::Topicable,
{
    ///
    pub fn new() -> Self {
        Default::default()
    }

    ///
    pub fn with_liveliness_lost(mut self, callback: fn(&crate::Writer<T>, LivelinessLost)) -> Self {
        self.liveliness_lost = Some(callback);
        self
    }

    ///
    pub fn with_offered_deadline_missed(
        mut self,
        callback: fn(&crate::Writer<T>, OfferedDeadlineMissed),
    ) -> Self {
        self.offered_deadline_missed = Some(callback);
        self
    }

    ///
    pub fn with_offered_incompatible_qos(
        mut self,
        callback: fn(&crate::Writer<T>, OfferedIncompatibleQoS),
    ) -> Self {
        self.offered_incompatible_qos = Some(callback);
        self
    }

    ///
    pub fn with_publication_matched(
        mut self,
        callback: fn(&crate::Writer<T>, PublicationMatched),
    ) -> Self
    where
        T: crate::Topicable,
    {
        self.publication_matched = Some(callback);
        self
    }

    ///
    #[inline]
    pub(crate) fn as_ffi(&self) -> Result<ffi::Listener> {
        ffi::Listener::new().map(|mut listener| {
            self.apply_listener_ffi(&mut listener);
            listener
        })
    }

    ///
    #[inline]
    pub(crate) fn apply_listener_ffi(&self, listener: &mut ffi::Listener) {
        if let Some(callback) = self.liveliness_lost {
            ffi::dds_listener_set_liveliness_lost(listener, callback);
        }
        if let Some(callback) = self.offered_deadline_missed {
            ffi::dds_listener_set_offered_deadline_missed(listener, callback);
        }
        if let Some(callback) = self.offered_incompatible_qos {
            ffi::dds_listener_set_offered_incompatible_qos(listener, callback);
        }
        if let Some(callback) = self.publication_matched {
            ffi::dds_listener_set_publication_matched(listener, callback);
        }
    }
}

impl<T> AsRef<ReaderListener<T>> for ReaderListener<T>
where
    T: crate::Topicable,
{
    fn as_ref(&self) -> &ReaderListener<T> {
        self
    }
}
impl<T> AsRef<WriterListener<T>> for WriterListener<T>
where
    T: crate::Topicable,
{
    fn as_ref(&self) -> &WriterListener<T> {
        self
    }
}
impl AsRef<SubscriberListener> for SubscriberListener {
    fn as_ref(&self) -> &SubscriberListener {
        self
    }
}
impl AsRef<PublisherListener> for PublisherListener {
    fn as_ref(&self) -> &PublisherListener {
        self
    }
}
impl<T> AsRef<TopicListener<T>> for TopicListener<T>
where
    T: crate::Topicable,
{
    fn as_ref(&self) -> &TopicListener<T> {
        self
    }
}
impl AsRef<Listener> for Listener {
    fn as_ref(&self) -> &Listener {
        self
    }
}

// impl<T> AsRef<ReaderListener<T>> for Listener<T> {
//     fn as_ref(&self) -> &ReaderListener<T> {
//         &self.subscriber.reader
//     }
// }
// impl<T> AsRef<WriterListener<T>> for Listener<T> {
//     fn as_ref(&self) -> &WriterListener<T> {
//         &self.publisher.writer
//     }
// }
impl AsRef<SubscriberListener> for Listener {
    fn as_ref(&self) -> &SubscriberListener {
        &self.subscriber
    }
}
impl AsRef<PublisherListener> for Listener {
    fn as_ref(&self) -> &PublisherListener {
        &self.publisher
    }
}
// impl<T> AsRef<TopicListener<T>> for Listener<T> {
//     fn as_ref(&self) -> &TopicListener<T> {
//         &self.topic
//     }
// }

// impl<T> AsRef<ReaderListener<T>> for SubscriberListener<T> {
//     fn as_ref(&self) -> &ReaderListener<T> {
//         &self.reader
//     }
// }
// impl<T> AsRef<WriterListener<T>> for PublisherListener<T> {
//     fn as_ref(&self) -> &WriterListener<T> {
//         &self.writer
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn receive_listener<L>(listener: L)
    where
        L: AsRef<Listener>,
    {
        let _ = listener.as_ref();
        assert!(true);
    }

    fn receive_topic_listener<L, T>(listener: L)
    where
        L: AsRef<TopicListener<T>>,
        T: crate::Topicable,
    {
        let _ = listener.as_ref();
        assert!(true);
    }

    fn receive_subscriber_listener<L>(listener: L)
    where
        L: AsRef<SubscriberListener>,
    {
        let _ = listener.as_ref();
        assert!(true);
    }

    fn receive_publisher_listener<L>(listener: L)
    where
        L: AsRef<PublisherListener>,
    {
        let _ = listener.as_ref();
        assert!(true);
    }

    fn receive_reader_listener<L, T>(listener: L)
    where
        L: AsRef<ReaderListener<T>>,
        T: crate::Topicable,
    {
        let _ = listener.as_ref();
        assert!(true);
    }

    fn receive_writer_listener<L, T>(listener: L)
    where
        L: AsRef<WriterListener<T>>,
        T: crate::Topicable,
    {
        let _ = listener.as_ref();
        assert!(true);
    }

    #[test]
    fn test_listener_create() {
        let listener = Listener::new()
            // .with_topic(|topic| topic.with_inconsistent_topic(|_, _| ()))
            .with_subscriber(|subscriber| {
                subscriber.with_data_on_readers(|_| ())
                // .with_reader(|reader| {
                //     reader
                //         .with_data_available(|_| ())
                //         .with_liveliness_changed(|_, _| ())
                //         .with_requested_deadline_missed(|_, _| ())
                //         .with_requested_incompatible_qos(|_, _| ())
                //         .with_sample_lost(|_, _| ())
                //         .with_sample_rejected(|_, _| ())
                //         .with_subscription_matched(|_, _| ())
                // })
            })
            .with_publisher(|publisher| {
                publisher
                //     .with_writer(|writer| {
                //     writer
                //         .with_liveliness_lost(|_, _| ())
                //         .with_offered_deadline_missed(|_, _| ())
                //         .with_offered_incompatible_qos(|_, _| ())
                //         .with_publication_matched(|_, _| ())
                // })
            });
        let topic_listener =
            TopicListener::<crate::tests::topic::Data>::new().with_inconsistent_topic(|_, _| ());
        let subscriber_listener = SubscriberListener::new()
            .with_data_on_readers(|_| ())
            // .with_reader(|reader| {
            //     reader
            //         .with_data_available(|_| ())
            //         .with_liveliness_changed(|_, _| ())
            //         .with_requested_deadline_missed(|_, _| ())
            //         .with_requested_incompatible_qos(|_, _| ())
            //         .with_sample_lost(|_, _| ())
            //         .with_sample_rejected(|_, _| ())
            //         .with_subscription_matched(|_, _| ())
            // })
            ;
        let publisher_listener =
            PublisherListener::new()
            // .with_writer(|writer| {
            //     writer
            //         .with_liveliness_lost(|_, _| ())
            //         .with_offered_deadline_missed(|_, _| ())
            //         .with_offered_incompatible_qos(|_, _| ())
            //         .with_publication_matched(|_, _| ())
            // })
            ;
        let reader_listener = ReaderListener::<crate::tests::topic::Data>::new()
            .with_data_available(|_| ())
            .with_liveliness_changed(|_, _| ())
            .with_requested_deadline_missed(|_, _| ())
            .with_requested_incompatible_qos(|_, _| ())
            .with_sample_lost(|_, _| ())
            .with_sample_rejected(|_, _| ())
            .with_subscription_matched(|_, _| ());
        let writer_listener = WriterListener::<crate::tests::topic::Data>::new()
            .with_liveliness_lost(|_, _| ())
            .with_offered_deadline_missed(|_, _| ())
            .with_offered_incompatible_qos(|_, _| ())
            .with_publication_matched(|_, _| ());

        receive_listener(&listener);

        receive_topic_listener(&topic_listener);
        // receive_topic_listener(&listener);

        receive_subscriber_listener(&subscriber_listener);
        receive_subscriber_listener(&listener);

        receive_publisher_listener(&publisher_listener);
        receive_publisher_listener(&listener);

        receive_reader_listener(&reader_listener);
        // receive_reader_listener(&subscriber_listener);
        // receive_reader_listener(&listener);

        receive_writer_listener(&writer_listener);
        // receive_writer_listener(&publisher_listener);
        // receive_writer_listener(&listener);
    }
}
