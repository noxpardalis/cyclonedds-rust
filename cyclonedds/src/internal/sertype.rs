//! The [`Sertype`] represents the extension point to thread through language-binding specific
//! behavior through Cyclone.
//!
//! The name of the corresponding type, whether the associated topic is keyed, and the
//! set of interface functions are registered with Cyclone through the
//! [`cyclonedds_sys::ddsi_sertype`].

use crate::internal::ffi;

/// The extension point for wrapping [`cyclonedds_sys::ddsi_sertype`].
#[repr(C)]
#[derive(Debug)]
pub struct Sertype<T> {
    pub(crate) inner: cyclonedds_sys::ddsi_sertype,
    phantom: std::marker::PhantomData<T>,
}

impl<T> Sertype<T>
where
    T: std::clone::Clone
        + serde::ser::Serialize
        + serde::de::DeserializeOwned
        + std::default::Default
        + std::fmt::Debug,
{
    pub(crate) const SERTYPE_OPS: cyclonedds_sys::ddsi_sertype_ops =
        cyclonedds_sys::ddsi_sertype_ops {
            version: ffi::sertype_ops::VERSION,
            arg: ffi::sertype_ops::ARG,
            free: Some(ffi::sertype_ops::free::<T>),
            zero_samples: Some(ffi::sertype_ops::zero_samples::<T>),
            realloc_samples: Some(ffi::sertype_ops::realloc_samples::<T>),
            free_samples: Some(ffi::sertype_ops::free_samples::<T>),
            equal: Some(ffi::sertype_ops::equal::<T>),
            hash: Some(ffi::sertype_ops::hash::<T>),

            // TODO integrate typelib feature?
            // type_id: Some(ffi::sertype_ops::type_id::<T>),
            // type_map: Some(ffi::sertype_ops::type_map::<T>),
            // type_info: Some(ffi::sertype_ops::type_info::<T>),
            // derive_sertype: Some(ffi::sertype_ops::derive_sertype::<T>),
            type_id: None,
            type_map: None,
            type_info: None,
            derive_sertype: None,

            get_serialized_size: Some(ffi::sertype_ops::get_serialized_size::<T>),
            serialize_into: Some(ffi::sertype_ops::serialize_into::<T>),
        };
    pub(crate) const SERDATA_OPS: cyclonedds_sys::ddsi_serdata_ops =
        cyclonedds_sys::ddsi_serdata_ops {
            eqkey: Some(ffi::serdata_ops::eqkey::<T>),
            get_size: Some(ffi::serdata_ops::get_size::<T>),
            from_ser: Some(ffi::serdata_ops::from_ser::<T>),
            from_ser_iov: Some(ffi::serdata_ops::from_ser_iov::<T>),
            from_keyhash: Some(ffi::serdata_ops::from_keyhash::<T>),
            from_sample: Some(ffi::serdata_ops::from_sample::<T>),
            to_ser: Some(ffi::serdata_ops::to_ser::<T>),
            to_ser_ref: Some(ffi::serdata_ops::to_ser_ref::<T>),
            to_ser_unref: Some(ffi::serdata_ops::to_ser_unref::<T>),
            to_sample: Some(ffi::serdata_ops::to_sample::<T>),
            to_untyped: Some(ffi::serdata_ops::to_untyped::<T>),
            untyped_to_sample: Some(ffi::serdata_ops::untyped_to_sample::<T>),
            free: Some(ffi::serdata_ops::free::<T>),
            print: Some(ffi::serdata_ops::print::<T>),
            get_keyhash: Some(ffi::serdata_ops::get_keyhash::<T>),
            from_loaned_sample: Some(ffi::serdata_ops::from_loaned_sample::<T>),
            from_psmx: Some(ffi::serdata_ops::from_psmx::<T>),
        };

    /// Create a new [`Sertype<T>`].
    pub fn new(type_name: &std::ffi::CStr, topic_kind_no_key: bool) -> Box<Self> {
        let inner = ffi::ddsi_sertype_new(
            type_name,
            &Self::SERTYPE_OPS,
            &Self::SERDATA_OPS,
            topic_kind_no_key,
        );

        Box::new(Sertype {
            inner,
            phantom: std::marker::PhantomData,
        })
    }
}

impl<T> Drop for Sertype<T> {
    fn drop(&mut self) {
        ffi::ddsi_sertype_unref(&mut self.inner);
        ffi::ddsi_sertype_fini(&mut self.inner);
    }
}
