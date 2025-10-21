//!

use crate::internal::serdata::SampleOrKey as SampleOrKeyInner;
use crate::{CdrBounds, CdrSize};
use md5::Digest;

///
pub struct KeyHash(pub(crate) [u8; 16]);

impl KeyHash {
    ///
    pub fn from_key<T: Topicable>(key: &T::Key, force_md5: bool) -> Option<KeyHash> {
        let mut serialized = cdr_encoding::to_vec::<_, byteorder::BigEndian>(&key).ok()?;

        let max_possible_serialized_size = T::Key::max_serialized_cdr_size();

        let key_hash = if force_md5 || max_possible_serialized_size > CdrSize::Bounded(16) {
            // The key hash should be computed via MD5.
            let mut hasher = md5::Md5::new();
            hasher.update(serialized);
            let hash = hasher.finalize();
            hash.into()
        } else {
            // The CDR serialized form fits and can be used as the key hash but
            // it must be padded to 16 bytes and those padding bytes must be zeroed.
            serialized.resize(16, 0);
            serialized.try_into().ok()?
        };

        Some(KeyHash(key_hash))
    }
}

///
pub trait Topicable:
    std::clone::Clone + std::fmt::Debug + serde::ser::Serialize + serde::de::DeserializeOwned
{
    /// TODO: If and when associated type defaults are stabilized this can be
    /// defaulted to `()`
    /// https://doc.rust-lang.org/beta/unstable-book/language-features/associated-type-defaults.html
    type Key: std::cmp::PartialOrd
        + std::cmp::PartialEq
        + std::fmt::Debug
        + serde::de::DeserializeOwned
        + serde::ser::Serialize
        + std::clone::Clone
        + std::default::Default
        + std::hash::Hash
        + crate::CdrBounds;

    ///
    const IS_KEYED: bool = std::mem::size_of::<Self::Key>() != 0;

    ///
    fn from_key(key: &Self::Key) -> Self;

    fn into_key(&self) -> Self::Key;

    ///
    fn type_name() -> String {
        let full_type_path = std::any::type_name::<Self>();

        // Strip out the leading module if it exists.
        if let Some((_, type_path)) = full_type_path.split_once("::") {
            type_path.to_string()
        } else {
            // There is no leading module so the full type path is just the type
            // name.
            full_type_path.to_string()
        }
    }
}

///
pub struct SampleOrKey<T>
where
    T: Topicable,
{
    inner: SampleOrKeyInner<T>,
    info: Info,
}

impl<T> std::clone::Clone for SampleOrKey<T>
where
    T: Topicable + std::clone::Clone,
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
    T: Topicable + std::fmt::Debug,
    T::Key: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.inner {
            SampleOrKeyInner::Sample { sample, .. } => f
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
    T: Topicable + std::cmp::PartialEq,
    T::Key: std::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        (match (&self.inner, &other.inner) {
            (
                SampleOrKeyInner::Sample { sample: lhs, .. },
                SampleOrKeyInner::Sample { sample: rhs, .. },
            ) => lhs == rhs,
            (SampleOrKeyInner::Key { key: lhs, .. }, SampleOrKeyInner::Key { key: rhs, .. }) => {
                lhs == rhs
            }
            _ => false,
        }) && self.info == other.info
    }
}

impl<T> std::cmp::Eq for SampleOrKey<T>
where
    T: Topicable + std::cmp::Eq,
    T::Key: std::cmp::Eq,
{
}

impl<T> std::hash::Hash for SampleOrKey<T>
where
    T: Topicable + std::hash::Hash,
    T::Key: std::hash::Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match &self.inner {
            SampleOrKeyInner::Sample { sample, .. } => sample.hash(state),
            SampleOrKeyInner::Key { key, .. } => key.hash(state),
        }
        self.info.hash(state);
    }
}

impl<T> SampleOrKey<T>
where
    T: Topicable,
{
    ///
    pub(crate) fn new_sample(sample: T, info: Info) -> Self {
        let inner = SampleOrKeyInner::Sample {
            sample: Box::new(sample),
            materialized_key: Default::default(),
        };
        Self { inner, info }
    }
    ///
    pub(crate) fn new_key(key: T::Key, info: Info) -> Self {
        let inner = SampleOrKeyInner::Key {
            key: Box::new(key),
            materialized_sample: Default::default(),
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
            SampleOrKeyInner::Key { .. } => false,
            SampleOrKeyInner::Sample { sample, .. } => f(sample),
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
    ///
    pub fn view(&self) -> View<'_, T> {
        match &self.inner {
            SampleOrKeyInner::Sample { sample, .. } => View::Sample(sample.as_ref()),
            SampleOrKeyInner::Key { key, .. } => View::Key(key.as_ref()),
        }
    }
}

///
pub enum View<'sample, T>
where
    T: Topicable,
{
    ///
    Sample(&'sample T),
    ///
    Key(&'sample T::Key),
}

impl<T> std::ops::Deref for SampleOrKey<T>
where
    T: Topicable,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match &self.inner {
            SampleOrKeyInner::Sample { sample, .. } => sample,
            SampleOrKeyInner::Key {
                key,
                materialized_sample,
            } => materialized_sample.get_or_init(|| Box::new(T::from_key(&key))),
        }
    }
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
