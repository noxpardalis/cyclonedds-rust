//!

///
pub trait Keyed: std::default::Default {
    /// TODO: If and when associated type defaults are stabilized this can be
    /// defaulted to the Never type via `std::convert::Infallible`.
    /// https://doc.rust-lang.org/beta/unstable-book/language-features/associated-type-defaults.html
    type Key;

    ///
    fn from_key(key: &Self::Key) -> Self;

    ///
    fn into_key(self: Self) -> Self::Key;
}

///
pub struct SampleOrKey<T>
where
    T: Keyed,
{
    inner: SampleOrKeyInner<T>,
    info: Info,
}

impl<T> std::clone::Clone for SampleOrKey<T>
where
    T: Keyed + std::clone::Clone,
    T::Key: std::clone::Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            info: self.info.clone(),
        }
    }
}

impl<T> std::fmt::Debug for SampleOrKey<T>
where
    T: Keyed + std::fmt::Debug,
    T::Key: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.inner {
            SampleOrKeyInner::Sample(sample) => f
                .debug_struct("SampleOrKey")
                .field("sample", sample)
                .field("info", &self.info)
                .finish(),
            SampleOrKeyInner::Key { key, .. } => f
                .debug_struct("SampleOrKey")
                .field("key", key)
                .field("info", &self.info)
                .finish(),
        }
    }
}

impl<T> std::cmp::PartialEq for SampleOrKey<T>
where
    T: Keyed + std::cmp::PartialEq,
    T::Key: std::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        (match (&self.inner, &other.inner) {
            (SampleOrKeyInner::Sample(lhs), SampleOrKeyInner::Sample(rhs)) => lhs == rhs,
            (SampleOrKeyInner::Key { key: lhs, .. }, SampleOrKeyInner::Key { key: rhs, .. }) => {
                lhs == rhs
            }
            _ => false,
        }) && self.info == other.info
    }
}

impl<T> std::cmp::Eq for SampleOrKey<T>
where
    T: Keyed + std::cmp::Eq,
    T::Key: std::cmp::Eq,
{
}

impl<T> std::hash::Hash for SampleOrKey<T>
where
    T: Keyed + std::hash::Hash,
    T::Key: std::hash::Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match &self.inner {
            SampleOrKeyInner::Sample(sample) => sample.hash(state),
            SampleOrKeyInner::Key { key, .. } => key.hash(state),
        }
        self.info.hash(state);
    }
}

impl<T> SampleOrKey<T>
where
    T: Keyed,
{
    ///
    pub(crate) fn new_sample(sample: T, info: Info) -> Self {
        let inner = SampleOrKeyInner::Sample(sample);
        Self { inner, info }
    }
    ///
    pub(crate) fn new_key(key: T::Key, info: Info) -> Self {
        let inner = SampleOrKeyInner::Key {
            key,
            materialized: Default::default(),
        };
        Self { inner, info }
    }
    ///
    pub fn info(&self) -> &Info {
        &self.info
    }
    ///
    pub fn sample(&self) -> Option<&T> {
        match &self.inner {
            SampleOrKeyInner::Sample(sample) => Some(sample),
            SampleOrKeyInner::Key { .. } => None,
        }
    }
    ///
    pub fn into_sample(self) -> Option<T> {
        match self.inner {
            SampleOrKeyInner::Sample(sample) => Some(sample),
            SampleOrKeyInner::Key { .. } => None,
        }
    }
    ///
    pub fn is_sample(&self) -> bool {
        matches!(self.inner, SampleOrKeyInner::Sample(..))
    }
    ///
    pub fn key(&self) -> Option<&T::Key> {
        match &self.inner {
            SampleOrKeyInner::Sample(..) => None,
            SampleOrKeyInner::Key { key, .. } => Some(key),
        }
    }
    ///
    pub fn into_key(self) -> Option<T::Key> {
        match self.inner {
            SampleOrKeyInner::Sample(..) => None,
            SampleOrKeyInner::Key { key, .. } => Some(key),
        }
    }
    ///
    pub fn is_key(&self) -> bool {
        matches!(self.inner, SampleOrKeyInner::Key { .. })
    }
    ///
    pub fn view(&self) -> View<'_, T> {
        match &self.inner {
            SampleOrKeyInner::Sample(sample) => View::Sample(sample),
            SampleOrKeyInner::Key { key, .. } => View::Key(key),
        }
    }
}

#[derive(Clone)]
enum SampleOrKeyInner<T>
where
    T: Keyed,
{
    Sample(T),
    Key {
        key: T::Key,
        materialized: std::cell::OnceCell<T>,
    },
}

///
pub enum View<'sample, T>
where
    T: Keyed,
{
    ///
    Sample(&'sample T),
    ///
    Key(&'sample T::Key),
}

impl<T> std::ops::Deref for SampleOrKey<T>
where
    T: Keyed,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match &self.inner {
            SampleOrKeyInner::Sample(sample) => sample,
            SampleOrKeyInner::Key { key, materialized } => {
                materialized.get_or_init(|| T::from_key(&key))
            }
        }
    }
}

///
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Info {
    ///
    pub state: crate::State,
    ///
    pub valid: bool,
    ///
    pub source_timestamp: crate::Time,
    ///
    pub instance_handle: crate::entity::InstanceHandle,
    ///
    pub publication_handle: crate::entity::InstanceHandle,
    ///
    pub disposed_generation_count: u32,
    ///
    pub no_writers_generation_count: u32,
    ///
    pub sample_rank: u32,
    ///
    pub generation_rank: u32,
    ///
    pub absolute_generation_rank: u32,
}

impl From<&cyclonedds_sys::dds_sample_info> for Info {
    fn from(sample_info: &cyclonedds_sys::dds_sample_info) -> Self {
        let state = crate::State::from_bits_truncate(sample_info.sample_state)
            | crate::State::from_bits_truncate(sample_info.view_state)
            | crate::State::from_bits_truncate(sample_info.instance_state);
        let valid = sample_info.valid_data;
        let instance_handle = crate::entity::InstanceHandle {
            inner: sample_info.instance_handle,
        };
        let publication_handle = crate::entity::InstanceHandle {
            inner: sample_info.publication_handle,
        };
        let source_timestamp = crate::Time::from_nanos(sample_info.source_timestamp);

        let disposed_generation_count = sample_info.disposed_generation_count;
        let no_writers_generation_count = sample_info.no_writers_generation_count;
        let sample_rank = sample_info.sample_rank;
        let generation_rank = sample_info.generation_rank;
        let absolute_generation_rank = sample_info.absolute_generation_rank;

        Self {
            state,
            valid,
            instance_handle,
            publication_handle,
            source_timestamp,
            disposed_generation_count,
            no_writers_generation_count,
            sample_rank,
            generation_rank,
            absolute_generation_rank,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_ref() {
        let info = Info {
            state: crate::State::empty(),
            valid: Default::default(),
            source_timestamp: Default::default(),
            instance_handle: crate::entity::InstanceHandle { inner: 0 },
            publication_handle: crate::entity::InstanceHandle { inner: 0 },
            disposed_generation_count: Default::default(),
            no_writers_generation_count: Default::default(),
            sample_rank: Default::default(),
            generation_rank: Default::default(),
            absolute_generation_rank: Default::default(),
        };
        let data = crate::tests::topic::Data {
            x: 10,
            y: 11,
            message: "sample".to_string(),
        };
        let sample = SampleOrKey::new_sample(data.clone(), info);

        assert_eq!(sample.info(), &info);
        assert_eq!(*sample, data);
        assert_eq!(sample.into_sample().unwrap(), data);
    }
}
