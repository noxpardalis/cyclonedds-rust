//! The [`Serdata`] represents the extension point for language-bindings to
//! interact with serialized sample data in Cyclone.

use crate::internal::ffi;
use crate::internal::sertype::Sertype;

/// The extension point for wrapping [`cyclonedds_sys::ddsi_serdata`].
#[repr(C)]
#[derive(Debug)]
pub struct Serdata<T> {
    pub(crate) inner: cyclonedds_sys::ddsi_serdata,
    pub(crate) sample: Option<std::sync::Arc<T>>,
    pub(crate) serialized_sample: Option<Vec<u8>>,
    pub(crate) key: Option<[u8; 16]>,
    kind: Kind,
}

impl<T> Serdata<T> {
    /// Create a new [`Serdata`] of a specific kind associated with a corresponding [`Sertype`].
    pub fn new(sertype: &Sertype<T>, kind: Kind) -> Box<Self> {
        let inner = ffi::ddsi_serdata_new(&sertype.inner, kind.into());

        Box::new(Self {
            inner,
            sample: None,
            serialized_sample: None,
            key: None,
            kind,
        })
    }

    /// Get a reference to the sample stored by the [`Serdata`].
    pub fn sample(&mut self) -> Option<&T> {
        if let Some(arc) = &self.sample {
            Some(arc.as_ref())
        } else {
            None
        }
    }

    /// Get the kind associated with this serdata.
    pub fn kind(&self) -> Kind {
        self.kind
    }
}

/// The possible states for a [`Serdata`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Kind {
    /// Is empty.
    Empty,
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
            cyclonedds_sys::ddsi_serdata_kind_SDK_EMPTY => Ok(Kind::Empty),
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
            Kind::Empty => cyclonedds_sys::ddsi_serdata_kind_SDK_EMPTY,
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
        let sertype = Sertype::<i32>::new(c"Data", true);

        let mut serdata = Serdata::new(&sertype, Kind::Empty);
        assert_eq!(serdata.sample(), None);

        let mut serdata = Serdata::new(&sertype, Kind::Key);
        assert_eq!(serdata.sample(), None);

        let mut serdata = Serdata::new(&sertype, Kind::Data);
        assert_eq!(serdata.sample(), None);

        let data = 106;
        serdata.sample = Some(std::sync::Arc::new(data));
        assert_eq!(serdata.sample(), Some(&data));
    }

    #[test]
    fn test_serdata_kind_conversion() {
        let actual = cyclonedds_sys::ddsi_serdata_kind_SDK_EMPTY.try_into();
        let expected = Ok(Kind::Empty);
        assert_eq!(actual, expected);

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
