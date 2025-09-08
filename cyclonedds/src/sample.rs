//!

///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sample<T> {
    ///
    pub(crate) sample: T,
    ///
    pub(crate) info: Info,
}

impl<T> Sample<T> {
    ///
    pub fn info(&self) -> Info {
        self.info
    }

    ///
    pub fn into_inner(self) -> T {
        self.sample
    }
}

impl<T> std::ops::Deref for Sample<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.sample
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
        let data = 10;
        let sample = Sample {
            sample: data,
            info: info,
        };

        assert_eq!(sample.info(), info);
        assert_eq!(*sample, data);
        assert_eq!(sample.into_inner(), data);
    }
}
