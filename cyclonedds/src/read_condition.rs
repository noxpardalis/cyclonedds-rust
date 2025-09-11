use crate::internal::ffi;
use crate::{Reader, Result, State};

///
#[derive(Debug)]
pub struct ReadCondition<'domain, 'participant, 'topic, 'reader, T>
where
    T: crate::Topicable,
{
    pub(crate) inner: cyclonedds_sys::dds_entity_t,
    phantom: std::marker::PhantomData<&'reader Reader<'domain, 'participant, 'topic, T>>,
}

impl<'d, 'p, 't, 'r, T> ReadCondition<'d, 'p, 't, 'r, T>
where
    T: crate::Topicable,
{
    ///
    pub fn new(reader: &'r Reader<'d, 'p, 't, T>, mask: State) -> Result<Self> {
        let inner = ffi::dds_create_readcondition(reader.inner, mask.bits())?;
        Ok(Self {
            inner,
            phantom: std::marker::PhantomData,
        })
    }

    ///
    pub fn mask(&self) -> Result<State> {
        let mask = ffi::dds_get_mask(self.inner)?;
        crate::state::State::from_bits(mask).ok_or(crate::error::Error::NonSpecific)
    }

    ///
    pub fn triggered(&self) -> Result<bool> {
        ffi::dds_triggered(self.inner)
    }

    ///
    pub fn take(&self) -> Result<Vec<crate::sample::SampleOrKey<T>>>
    where
        T: std::clone::Clone,
    {
        ffi::dds_take(self.inner)
    }

    ///
    pub fn read(&self) -> Result<Vec<crate::sample::SampleOrKey<T>>>
    where
        T: std::clone::Clone,
    {
        ffi::dds_read(self.inner)
    }

    ///
    pub fn peek(&self) -> Result<Vec<crate::sample::SampleOrKey<T>>>
    where
        T: std::clone::Clone,
    {
        ffi::dds_peek(self.inner)
    }
}

impl<T> Drop for ReadCondition<'_, '_, '_, '_, T>
where
    T: crate::Topicable,
{
    fn drop(&mut self) {
        let result = ffi::dds_delete(self.inner);
        debug_assert!(result.is_ok());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state;

    #[test]
    fn test_read_condition_create() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain).unwrap();
        let topic =
            crate::Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();
        let reader = crate::Reader::new(&topic).unwrap();
        let _ = ReadCondition::new(
            &reader,
            state::sample::Any | state::instance::Any | state::view::Any,
        )
        .unwrap();
    }

    #[test]
    fn test_read_condition_create_with_invalid_reader() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain).unwrap();
        let topic =
            crate::Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();
        let mut reader = crate::Reader::new(&topic).unwrap();
        let reader_id = reader.inner;
        reader.inner = 0;
        let result = ReadCondition::new(
            &reader,
            state::sample::Any | state::instance::Any | state::view::Any,
        )
        .unwrap_err();
        reader.inner = reader_id;
        assert_eq!(result, crate::Error::BadParameter);
    }

    #[test]
    fn test_read_condition_get_mask() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain).unwrap();
        let topic =
            crate::Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();
        let reader = crate::Reader::new(&topic).unwrap();

        let mask = state::sample::Any | state::instance::Any | state::view::Any;

        let read_condition = ReadCondition::new(&reader, mask).unwrap();
        let result = read_condition.mask().unwrap();
        assert_eq!(result, mask);

        let mask = state::sample::Fresh | state::instance::Unregistered | state::view::Old;
        let result = read_condition.mask().unwrap();
        assert_ne!(result, mask);

        let read_condition = ReadCondition::new(&reader, mask).unwrap();
        let result = read_condition.mask().unwrap();
        assert_eq!(result, mask);
    }

    #[test]
    fn test_read_condition_get_mask_on_invalid_read_condition() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain).unwrap();
        let topic =
            crate::Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();
        let reader = crate::Reader::new(&topic).unwrap();
        let mut read_condition = ReadCondition::new(
            &reader,
            state::sample::Any | state::instance::Any | state::view::Any,
        )
        .unwrap();
        let read_condition_id = read_condition.inner;
        read_condition.inner = 0;
        let result = read_condition.mask().unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);
        let result = read_condition.triggered().unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);
        read_condition.inner = read_condition_id;
    }

    #[test]
    fn test_read_condition_triggering_reads() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain).unwrap();
        let topic =
            crate::Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();
        let reader = crate::Reader::new(&topic).unwrap();
        let writer = crate::Writer::new(&topic).unwrap();

        let mask = state::sample::Stale | state::instance::Any | state::view::Any;

        let read_condition = ReadCondition::new(&reader, mask).unwrap();

        let sample = crate::tests::topic::Data {
            x: 101,
            y: 202,
            message: "hello".to_string(),
        };
        writer.write(&sample).unwrap();

        let read_condition_received = read_condition.read().unwrap();
        assert_eq!(read_condition_received.len(), 0);
        let triggered = read_condition.triggered().unwrap();
        assert_eq!(triggered, false);

        let reader_received = reader.read().unwrap();
        assert_eq!(reader_received.len(), 1);
        assert_eq!(*reader_received[0], sample);
        assert_eq!(
            reader_received[0].info().state,
            state::sample::Fresh | state::view::New | state::instance::Alive
        );

        let triggered = read_condition.triggered().unwrap();
        assert_eq!(triggered, true);

        let read_condition_received = read_condition.peek().unwrap();
        assert_eq!(read_condition_received.len(), 1);
        assert_eq!(*read_condition_received[0], sample);

        let triggered = read_condition.triggered().unwrap();
        assert_eq!(triggered, true);

        let read_condition_received = read_condition.take().unwrap();
        assert_eq!(read_condition_received.len(), 1);
        assert_eq!(*read_condition_received[0], sample);

        let triggered = read_condition.triggered().unwrap();
        assert_eq!(triggered, false);

        let reader_received = reader.read().unwrap();
        assert!(reader_received.is_empty());

        let read_condition_received = read_condition.read().unwrap();
        assert!(read_condition_received.is_empty());
    }
}
