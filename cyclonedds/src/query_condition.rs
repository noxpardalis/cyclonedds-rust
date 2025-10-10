use crate::Reader;
use crate::Result;
use crate::internal::ffi;
use crate::state::State;
use ffi::Filter;

/// A condition ...
///
/// The closure provided to the [`QueryCondition`] must be *zero-sized*.
///
/// This restriction comes from how the callback is integrated with the
/// underlying C API. For ergonomics, the closure is reconstructed inside a
/// wrapper that converts raw C types into Rust types before invoking it. That
/// reconstruction depends only on the closure’s type, so it cannot rely on any
/// captured state.
///
/// In practice, this means the callback must be either:
///
///   - a [function item](https://doc.rust-lang.org/reference/types/function-item.html),
///     which is always zero-sized, or
///   - a [closure](https://doc.rust-lang.org/reference/types/closure.html) that
///     does not capture any variables from its environment
///
/// The following example will **fail to compile** because the closure captures
/// `x`, making it non–zero-sized:
///
/// ```compile_fail
/// # use cyclonedds as dds;
/// # use dds::state;
/// #
/// # #[derive(serde::Serialize, serde::Deserialize, Clone)]
/// # struct Data {
/// #     x: i32,
/// # }
/// # fn create_your_reader() -> dds::Reader<'static, 'static, 'static, Data> {
/// #     unimplemented!()
/// # }
/// # let reader: dds::Reader<Data> = create_your_reader();
/// let x = 10;
/// let result = dds::QueryCondition::new(
///     &reader,
///     state::sample::Any | state::instance::Any | state::view::Any,
///     // ❌ Error: closure captures `x`, so it is not zero-sized.
///     |sample| sample.x < x,
/// )?;
/// # Ok::<(), dds::Error>(())
/// ```
///
/// ✅ Instead, use a function item or a non-capturing closure, e.g.:
///
/// ```rust,ignore
/// |sample| sample.x < 10
/// ```
///
/// The compiler will emit an error similar to:
///
/// ```text
/// error[E0080]: evaluation panicked: the provided callback is not zero-sized
///   = note: closures that capture values from their environment are not zero-sized
///   = help: ensure the callback is either:
///           - a function item, e.g. `fn my_callback() {}`
///           - a closure that does not capture any external state
/// ```
///
/// This is enforced via an internal compile-time assertion
/// `assert!(size_of::<F>() == 0)` but note that the associated compiler output
/// can be quite lengthy.
///
/// <details>
/// <summary>Click to see a sample of the full compiler output</summary>
///
/// ```text
/// error[E0080]: evaluation panicked: the provided callback is not zero-sized
///                 = note: closures that capture values from their environment are not zero-sized
///                 = help: ensure the callback is either:
///                         - a function item, e.g. `fn my_callback() {}`
///                         - a closure that does not capture any external state
///    --> cyclonedds/src/internal/ffi.rs:605:49
///     |
/// 605 |       const IS_PROVIDED_CALLBACK_ZERO_SIZED: () = assert!(
///     |  _________________________________________________^
/// 606 | |         size_of::<F>() == 0,
/// 607 | |         "\
/// 608 | | the provided callback is not zero-sized
/// ...   |
/// 613 | | "
/// 614 | |     );
///     | |_____^ evaluation of `<query_condition::QueryCondition<...> as internal::ffi::Filter<...>>::IS_PROVIDED_CALLBACK_ZERO_SIZED` failed here
///
/// note: erroneous constant encountered
///   --> cyclonedds/src/query_condition.rs:84:17
///    |
/// 84 |         let _ = Self::IS_PROVIDED_CALLBACK_ZERO_SIZED;
///    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
///
/// ...
///
/// note: erroneous constant encountered
///    --> cyclonedds/src/internal/ffi.rs:572:13
///     |
/// 572 |     let _ = Callback::IS_PROVIDED_CALLBACK_ZERO_SIZED;
///     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
/// ```
/// </details>
pub struct QueryCondition<'domain, 'participant, 'topic, 'reader, T, F>
where
    F: Fn(&T) -> bool,
{
    pub(crate) inner: cyclonedds_sys::dds_entity_t,
    phantom_callback: std::marker::PhantomData<F>,
    phantom: std::marker::PhantomData<&'reader Reader<'topic, 'domain, 'participant, T>>,
}

impl<T, F> std::fmt::Debug for QueryCondition<'_, '_, '_, '_, T, F>
where
    F: Fn(&T) -> bool,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QueryCondition")
            .field("inner", &self.inner)
            .field("phantom", &self.phantom)
            .finish()
    }
}

impl<T, F> Filter<T, F> for QueryCondition<'_, '_, '_, '_, T, F>
where
    T: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    F: Fn(&T) -> bool,
{
}

impl<'d, 'p, 't, 'r, T, F> QueryCondition<'d, 'p, 't, 'r, T, F>
where
    T: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    F: Fn(&T) -> bool,
{
    ///
    pub fn new(reader: &'r Reader<'d, 'p, 't, T>, mask: State, _: F) -> Result<Self> {
        let _ = Self::IS_PROVIDED_CALLBACK_ZERO_SIZED;

        let inner = ffi::dds_create_querycondition::<T, F, Self>(reader.inner, mask.bits())?;
        Ok(Self {
            inner,
            phantom_callback: std::marker::PhantomData,
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
    pub fn take(&self) -> Result<Vec<Result<crate::sample::Sample<T>, crate::sample::Info>>>
    where
        T: std::clone::Clone,
    {
        ffi::dds_take(self.inner)
    }

    ///
    pub fn read(&self) -> Result<Vec<Result<crate::sample::Sample<T>, crate::sample::Info>>>
    where
        T: std::clone::Clone,
    {
        ffi::dds_read(self.inner)
    }

    ///
    pub fn peek(&self) -> Result<Vec<Result<crate::sample::Sample<T>, crate::sample::Info>>>
    where
        T: std::clone::Clone,
    {
        ffi::dds_peek(self.inner)
    }
}

impl<T, F> Drop for QueryCondition<'_, '_, '_, '_, T, F>
where
    F: Fn(&T) -> bool,
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

    fn query(_: &crate::tests::topic::Data) -> bool {
        true
    }

    #[test]
    fn test_query_condition_create() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain).unwrap();
        let topic =
            crate::Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();
        let reader = crate::Reader::new(&participant, &topic).unwrap();
        let _ = QueryCondition::new(
            &reader,
            state::sample::Any | state::instance::Any | state::view::Any,
            query,
        )
        .unwrap();
    }

    #[test]
    fn test_query_condition_create_with_invalid_reader() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain).unwrap();
        let topic =
            crate::Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();
        let mut reader = crate::Reader::new(&participant, &topic).unwrap();
        let reader_id = reader.inner;
        reader.inner = 0;
        let result = QueryCondition::new(
            &reader,
            state::sample::Any | state::instance::Any | state::view::Any,
            query,
        )
        .unwrap_err();
        reader.inner = reader_id;
        assert_eq!(result, crate::Error::BadParameter);
    }

    #[test]
    fn test_query_condition_debug_formatting() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain).unwrap();
        let topic =
            crate::Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();
        let reader = crate::Reader::new(&participant, &topic).unwrap();
        let query_condition = QueryCondition::new(
            &reader,
            state::sample::Any | state::instance::Any | state::view::Any,
            query,
        )
        .unwrap();

        let result = format!("{query_condition:?}");
        assert!(result.contains(&format!("{}", query_condition.inner)));
    }

    #[test]
    fn test_query_condition_get_mask() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain).unwrap();
        let topic =
            crate::Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();
        let reader = crate::Reader::new(&participant, &topic).unwrap();

        let mask = state::sample::Any | state::instance::Any | state::view::Any;

        let query_condition = QueryCondition::new(&reader, mask, query).unwrap();
        let result = query_condition.mask().unwrap();
        assert_eq!(result, mask);

        let mask = state::sample::Fresh | state::instance::Unregistered | state::view::Old;
        let result = query_condition.mask().unwrap();
        assert_ne!(result, mask);

        let read_condition = QueryCondition::new(&reader, mask, |_| false).unwrap();
        let result = read_condition.mask().unwrap();
        assert_eq!(result, mask);
    }

    #[test]
    fn test_query_condition_get_mask_on_invalid_query_condition() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain).unwrap();
        let topic =
            crate::Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();
        let reader = crate::Reader::new(&participant, &topic).unwrap();
        let mut query_condition = QueryCondition::new(
            &reader,
            state::sample::Any | state::instance::Any | state::view::Any,
            query,
        )
        .unwrap();
        let query_condition_id = query_condition.inner;
        query_condition.inner = 0;
        let result = query_condition.mask().unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);
        let result = query_condition.triggered().unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);
        query_condition.inner = query_condition_id;
    }

    #[test]
    fn test_query_condition_triggering_reads() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain).unwrap();
        let topic =
            crate::Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();
        let reader = crate::Reader::new(&participant, &topic).unwrap();
        let writer = crate::Writer::new(&participant, &topic).unwrap();

        let mask = state::sample::Stale | state::instance::Any | state::view::Any;

        let query_condition = QueryCondition::new(&reader, mask, query).unwrap();

        let sample = crate::tests::topic::Data {
            x: 101,
            y: 202,
            message: "hello".to_string(),
        };
        writer.write(&sample).unwrap();

        let query_condition_received = query_condition
            .read()
            .unwrap()
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(query_condition_received.len(), 0);
        let triggered = query_condition.triggered().unwrap();
        assert_eq!(triggered, false);

        let reader_received = reader
            .read()
            .unwrap()
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(reader_received.len(), 1);
        assert_eq!(*reader_received[0], sample);
        assert_eq!(
            reader_received[0].info().state,
            state::sample::Fresh | state::view::New | state::instance::Alive
        );

        let triggered = query_condition.triggered().unwrap();
        assert_eq!(triggered, true);

        let query_condition_received = query_condition
            .peek()
            .unwrap()
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(query_condition_received.len(), 1);
        assert_eq!(*query_condition_received[0], sample);

        let triggered = query_condition.triggered().unwrap();
        assert_eq!(triggered, true);

        let query_condition_received = query_condition
            .take()
            .unwrap()
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(query_condition_received.len(), 1);
        assert_eq!(*query_condition_received[0], sample);

        let triggered = query_condition.triggered().unwrap();
        assert_eq!(triggered, false);

        let reader_received = reader.read().unwrap();
        assert!(reader_received.is_empty());

        let query_condition_received = query_condition.read().unwrap();
        assert!(query_condition_received.is_empty());
    }

    #[test]
    fn test_query_condition_non_triggering_reads() {
        let domain_id = crate::tests::domain::unique_id();
        let domain = crate::Domain::new(domain_id).unwrap();
        let topic_name = crate::tests::topic::unique_name();
        let participant = crate::Participant::new(&domain).unwrap();
        let topic =
            crate::Topic::<crate::tests::topic::Data>::new(&participant, &topic_name).unwrap();
        let reader = crate::Reader::new(&participant, &topic).unwrap();
        let writer = crate::Writer::new(&participant, &topic).unwrap();

        let mask = state::sample::Stale | state::instance::Any | state::view::Any;

        let query_condition = QueryCondition::new(&reader, mask, |_| false).unwrap();

        let sample = crate::tests::topic::Data {
            x: 101,
            y: 202,
            message: "hello".to_string(),
        };
        writer.write(&sample).unwrap();

        let query_condition_received = query_condition.read().unwrap();
        assert!(query_condition_received.is_empty());
        let triggered = query_condition.triggered().unwrap();
        assert_eq!(triggered, false);

        let reader_received = reader
            .read()
            .unwrap()
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(reader_received.len(), 1);
        assert_eq!(*reader_received[0], sample);
        assert_eq!(
            reader_received[0].info().state,
            state::sample::Fresh | state::view::New | state::instance::Alive
        );

        let triggered = query_condition.triggered().unwrap();
        assert_eq!(triggered, false);

        let query_condition_received = query_condition.peek().unwrap();
        assert!(query_condition_received.is_empty());

        let triggered = query_condition.triggered().unwrap();
        assert_eq!(triggered, false);

        let query_condition_received = query_condition.take().unwrap();
        assert_eq!(query_condition_received.len(), 0);

        let triggered = query_condition.triggered().unwrap();
        assert_eq!(triggered, false);

        let reader_received = reader.read().unwrap();
        assert_eq!(reader_received.len(), 1);

        let query_condition_received = query_condition.read().unwrap();
        assert!(query_condition_received.is_empty());
    }
}
