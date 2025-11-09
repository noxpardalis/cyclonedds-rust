//!

use crate::Topicable;

#[derive(Clone, Debug)]
pub(crate) enum SampleOrKeyInner<T>
where
    T: crate::Topicable,
{
    Sample {
        sample: Box<T>,
        materialized_key: std::cell::OnceCell<Box<T::Key>>,
    },
    Key {
        key: Box<T::Key>,
        materialized_sample: std::cell::OnceCell<Box<T>>,
    },
}

impl<T> std::hash::Hash for SampleOrKeyInner<T>
where
    T: crate::Topicable + std::hash::Hash,
    T::Key: std::hash::Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // TODO should this always be based on the key-hash even in the zero-sized scenario?
        // If so the `std::hash::Hash` bounds can disappear.
        //
        // There are basically three options here:
        // (1):
        // `self.key().hash(state)` always in which case remove the hash bounds on T.
        // (2):
        // if `T::TOPIC_KIND_NO_KEY` base hash on value else on key
        // (3):
        // what is here now, base it purely on the hash of the specific variant (which seems the
        // least surprising).
        match self {
            SampleOrKeyInner::Sample { sample, .. } => sample.hash(state),
            SampleOrKeyInner::Key { key, .. } => key.hash(state),
        }
    }
}

impl<T> SampleOrKeyInner<T>
where
    T: crate::Topicable,
{
    pub fn new_sample(sample: T) -> Self {
        Self::Sample {
            sample: Box::new(sample),
            materialized_key: std::cell::OnceCell::new(),
        }
    }

    pub fn new_key(key: T::Key) -> Self {
        Self::Key {
            key: Box::new(key),
            materialized_sample: std::cell::OnceCell::new(),
        }
    }

    pub fn key(&self) -> &T::Key {
        match self {
            Self::Sample {
                sample,
                materialized_key,
            } => materialized_key.get_or_init(|| Box::new(sample.as_key())),
            Self::Key { key, .. } => key,
        }
    }

    pub fn sample(&self) -> &T {
        match self {
            Self::Sample { sample, .. } => sample,
            Self::Key {
                key,
                materialized_sample,
            } => materialized_sample.get_or_init(|| Box::new(T::from_key(key))),
        }
    }
}

///
pub struct SampleOrKey<T>
where
    T: crate::Topicable,
{
    ///
    inner: SampleOrKeyInner<T>,
    ///
    pub(crate) info: Info,
}

impl<T> std::clone::Clone for SampleOrKey<T>
where
    T: Topicable + std::clone::Clone,
    T::Key: std::clone::Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            info: self.info,
        }
    }
}

impl<T> std::fmt::Debug for SampleOrKey<T>
where
    T: Topicable + std::fmt::Debug,
    T::Key: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("SampleOrKey");

        let f = match &self.inner {
            SampleOrKeyInner::Sample { sample, .. } => f.field("sample", sample),
            SampleOrKeyInner::Key { key, .. } => f.field("key", key),
        };

        f.field("info", &self.info).finish()
    }
}

impl<T> SampleOrKey<T>
where
    T: crate::Topicable,
{
    ///
    pub(crate) fn new_sample(sample: T, info: Info) -> Self {
        let inner = SampleOrKeyInner::new_sample(sample);
        Self { inner, info }
    }

    ///
    pub(crate) fn new_key(key: T::Key, info: Info) -> Self {
        let inner = SampleOrKeyInner::new_key(key);
        Self { inner, info }
    }

    ///
    pub fn info(&self) -> &Info {
        &self.info
    }

    ///
    pub fn sample(&self) -> Option<&T> {
        match &self.inner {
            SampleOrKeyInner::Sample { sample, .. } => Some(sample),
            SampleOrKeyInner::Key { .. } => None,
        }
    }

    ///
    pub fn into_sample(self) -> Option<T> {
        match self.inner {
            SampleOrKeyInner::Sample { sample, .. } => Some(*sample),
            SampleOrKeyInner::Key { .. } => None,
        }
    }

    ///
    pub fn is_sample(&self) -> bool {
        matches!(self.inner, SampleOrKeyInner::Sample { .. })
    }

    ///
    pub fn is_sample_and(&self, f: impl FnOnce(&T) -> bool) -> bool {
        match &self.inner {
            SampleOrKeyInner::Sample { sample, .. } => f(sample),
            SampleOrKeyInner::Key { .. } => false,
        }
    }

    ///
    pub fn key(&self) -> Option<&T::Key> {
        match &self.inner {
            SampleOrKeyInner::Sample { .. } => None,
            SampleOrKeyInner::Key { key, .. } => Some(key),
        }
    }

    ///
    pub fn into_key(self) -> Option<T::Key> {
        match self.inner {
            SampleOrKeyInner::Sample { .. } => None,
            SampleOrKeyInner::Key { key, .. } => Some(*key),
        }
    }

    ///
    pub fn is_key(&self) -> bool {
        matches!(self.inner, SampleOrKeyInner::Key { .. })
    }

    ///
    pub fn is_key_and(&self, f: impl FnOnce(&T::Key) -> bool) -> bool {
        match &self.inner {
            SampleOrKeyInner::Sample { .. } => false,
            SampleOrKeyInner::Key { key, .. } => f(key),
        }
    }

    pub fn view(&self) -> View<'_, T> {
        match &self.inner {
            SampleOrKeyInner::Sample { sample, .. } => View::Sample(sample.as_ref()),
            SampleOrKeyInner::Key { key, .. } => View::Key(key.as_ref()),
        }
    }
}

impl<T> std::ops::Deref for SampleOrKey<T>
where
    T: crate::Topicable,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match &self.inner {
            SampleOrKeyInner::Sample { sample, .. } => sample,
            SampleOrKeyInner::Key {
                key,
                materialized_sample,
            } => materialized_sample.get_or_init(|| Box::new(T::from_key(key))),
        }
    }
}

pub enum View<'sample, T>
where
    T: Topicable,
{
    ///
    Sample(&'sample T),
    ///
    Key(&'sample T::Key),
}

///
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Info {
    ///
    pub state: crate::State,
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
