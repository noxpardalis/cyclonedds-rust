//! The [`Serdata`] represents the extension point for language-bindings to
//! interact with serialized sample data in Cyclone.

use crate::internal::ffi;
use crate::internal::ffi::serdata_ops::DDSI_RTPS_HEADER_SIZE;
use crate::internal::sertype::Sertype;
use crate::internal::traits::{CdrHeader, Hash32};
use crate::sample::SampleOrKeyInner as SampleOrKey;

/// The extension point for wrapping [`cyclonedds_sys::ddsi_serdata`].
#[repr(C)]
#[derive(Debug)]
pub struct Serdata<T>
where
    T: crate::Topicable,
{
    pub(crate) inner: cyclonedds_sys::ddsi_serdata,
    pub(crate) serialized_sample: std::cell::OnceCell<Vec<u8>>,
    pub(crate) sample: std::sync::Arc<SampleOrKey<T>>,
}

impl<T> Serdata<T>
where
    T: crate::Topicable,
{
    /// Create a new [`Serdata`] of a specific kind associated with a corresponding [`Sertype`].
    pub fn new(sertype: &Sertype<T>, sample: SampleOrKey<T>) -> Box<Self> {
        let kind = match sample {
            SampleOrKey::Sample { .. } => Kind::Data,
            SampleOrKey::Key { .. } => Kind::Key,
        };
        let mut inner = ffi::ddsi_serdata_new(&sertype.inner, kind.into());
        inner.hash = sample.key().hash32();

        Box::new(Self {
            inner,
            sample: std::sync::Arc::new(sample),
            serialized_sample: std::cell::OnceCell::new(),
        })
    }

    /// Get a reference to the sample stored by the [`Serdata`].
    pub fn sample(&mut self) -> &T {
        self.sample.as_ref().sample()
    }

    pub fn key(&mut self) -> &T::Key {
        self.sample.as_ref().key()
    }

    pub fn serialized(&mut self) -> Result<&[u8], cdr_encoding::Error> {
        // NOTE: this initially used self.serialized_sample.get_or_init() but since the
        // serialization can fail this has to be done as follows.
        //
        // TODO: swap this out for `get_or_try_init()` when that is available.
        if let Some(serialized_sample) = self.serialized_sample.get() {
            Ok(serialized_sample)
        } else {
            let mut serialized: Vec<_> = byteorder::NativeEndian::cdr_header()
                .into_iter()
                .chain([0, 0])
                .collect();
            match self.sample.as_ref() {
                SampleOrKey::Sample { sample, .. } => {
                    cdr_encoding::to_writer::<_, byteorder::NativeEndian, _>(
                        &mut serialized,
                        sample,
                    )?;
                }
                SampleOrKey::Key { key, .. } => {
                    cdr_encoding::to_writer::<_, byteorder::NativeEndian, _>(&mut serialized, key)?;
                }
            }
            // SAFETY: guaranteed because in this branch the serialized sample is unset.
            self.serialized_sample.set(serialized).unwrap();
            // SAFETY: guaranteed because the set call was successful by here.
            Ok(self.serialized_sample.get().unwrap())
        }
    }

    pub fn serialized_with_size(&mut self, size: usize) -> &[u8] {
        if let Some(serialized_sample) = self.serialized_sample.get_mut() {
            serialized_sample.resize(size, 0);
        }

        self.serialized_sample.get_or_init(|| {
            let mut serialized = vec![0; size];
            serialized[0..=1].copy_from_slice(&byteorder::NativeEndian::cdr_header());
            match self.sample.as_ref() {
                SampleOrKey::Sample { sample, .. } => {
                    cdr_encoding::to_writer::<_, byteorder::NativeEndian, _>(
                        &mut serialized[DDSI_RTPS_HEADER_SIZE..],
                        sample,
                    )
                    .unwrap();
                }
                SampleOrKey::Key { key, .. } => {
                    cdr_encoding::to_writer::<_, byteorder::NativeEndian, _>(
                        &mut serialized[DDSI_RTPS_HEADER_SIZE..],
                        key,
                    )
                    .unwrap();
                }
            }
            serialized
        })
    }

    /// Get the kind associated with this serdata.
    pub fn kind(&self) -> Kind {
        match &self.sample.as_ref() {
            SampleOrKey::Sample { .. } => Kind::Data,
            SampleOrKey::Key { .. } => Kind::Key,
        }
    }
}

/// The possible states for a [`Serdata`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Kind {
    /// Contains the serialized key-hash.
    Key,
    /// Contains the serialized data.
    Data,
}

impl TryFrom<cyclonedds_sys::ddsi_serdata_kind> for Kind {
    type Error = crate::Error;

    #[inline]
    fn try_from(
        value: cyclonedds_sys::ddsi_serdata_kind,
    ) -> std::result::Result<Self, Self::Error> {
        match value {
            // Was added in OpenSlice when coherency support was added Because a
            // way to pass through just metadata was needed (perhaps just for
            // group coherency). For example, to send a message that this is the
            // end of a coherent set. This should not be exposed by the C
            // library.
            cyclonedds_sys::ddsi_serdata_kind_SDK_EMPTY => {
                unreachable!("SDK EMPTY should never be exposed by the C library")
            }
            cyclonedds_sys::ddsi_serdata_kind_SDK_KEY => Ok(Kind::Key),
            cyclonedds_sys::ddsi_serdata_kind_SDK_DATA => Ok(Kind::Data),
            _ => Err(crate::Error::BadParameter),
        }
    }
}

impl From<Kind> for cyclonedds_sys::ddsi_serdata_kind {
    #[inline]
    fn from(val: Kind) -> Self {
        match val {
            Kind::Key => cyclonedds_sys::ddsi_serdata_kind_SDK_KEY,
            Kind::Data => cyclonedds_sys::ddsi_serdata_kind_SDK_DATA,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_serdata_sample_retrieval() {
        // let sertype = Sertype::<i32>::new(c"Data", true);

        // let mut serdata = Serdata::new(&sertype, Kind::Key);
        // assert_eq!(serdata.sample(), None);

        // let mut serdata = Serdata::new(&sertype, Kind::Data);
        // assert_eq!(serdata.sample(), None);

        // let data = 106;
        // serdata.sample = Some(std::sync::Arc::new(data));
        // assert_eq!(serdata.sample(), Some(&data));
    }

    #[test]
    fn test_serdata_kind_conversion() {
        let actual = cyclonedds_sys::ddsi_serdata_kind_SDK_KEY.try_into();
        let expected = Ok(Kind::Key);
        assert_eq!(actual, expected);

        let actual = cyclonedds_sys::ddsi_serdata_kind_SDK_DATA.try_into();
        let expected = Ok(Kind::Data);
        assert_eq!(actual, expected);

        let actual: Result<Kind, _> = std::ffi::c_uint::MAX.try_into();
        let expected = Err(crate::Error::BadParameter);
        assert_eq!(actual, expected);
    }
}
