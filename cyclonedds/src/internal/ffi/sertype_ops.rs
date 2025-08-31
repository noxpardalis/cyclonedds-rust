//! Operations that allow Cyclone to interact with Rust allocated data structures.
//! These are threaded through the [`Sertype`] type.

use std::ffi::CStr;
use std::hash::{Hash, Hasher};

use crate::internal::sertype::Sertype;

/// A version to allow Cyclone DDS to ensure backwards compatibility if breaking changes to the
/// sertype API are introduced.
pub const VERSION: cyclonedds_sys::ddsi_sertype_v0_t = Some(cyclonedds_sys::ddsi_sertype_v0);

/// Arbitrary metadata that was originally used for backwards compatibility as part of the
/// `sertopic` -> `sertype` changeover.
pub const ARG: *mut std::ffi::c_void = std::ptr::null_mut();

/// A flag specifying that the data representation in use corresponds to XCDR1.
pub const DATA_REPRESENTATION_XCDR1: u32 = cyclonedds_sys::DDS_DATA_REPRESENTATION_XCDR1;
/// A flag specifying that the data representation in use corresponds to XCDR2.
pub const DATA_REPRESENTATION_XCDR2: u32 = cyclonedds_sys::DDS_DATA_REPRESENTATION_XCDR2;

/// Free a [`Sertype`] created on the Rust side of the FFI.
///
/// ## Safety
/// The provided `sertype` pointer must be from the pointer embedded in a [`Sertype`] that was
/// created via [`Sertype::new`].
pub unsafe extern "C" fn free<T>(sertype: *mut cyclonedds_sys::ddsi_sertype)
where
    T: crate::Topicable,
{
    let sertype = unsafe { Box::from_raw(sertype as *mut Sertype<T>) };
    drop(sertype);
}

///
/// TODO: validate what the purpose of this is in the actual functioning of Cyclone?
/// The C++ API also maps this to a no-op.
pub unsafe extern "C" fn zero_samples<T>(
    _sertype: *const cyclonedds_sys::ddsi_sertype,
    samples: *mut std::ffi::c_void,
    count: usize,
) where
    T: crate::Topicable,
{
    let samples = samples as *mut T;
    let default_sample = T::from_key(&T::Key::default());

    for i in 0..count {
        unsafe {
            let pointer = samples.add(i);
            pointer.write(default_sample.clone());
        }
    }
}

/// Realloc the sample buffer.
pub unsafe extern "C" fn realloc_samples<T>(
    pointers: *mut *mut std::ffi::c_void,
    _sertype: *const cyclonedds_sys::ddsi_sertype,
    old_samples: *mut std::ffi::c_void,
    old_count: usize,
    new_count: usize,
) where
    T: crate::Topicable,
{
    let pointers = pointers as *mut *mut T;
    let old_samples = old_samples as *mut T;

    // Copy over initial samples.
    if !old_samples.is_null() {
        let count = old_count.min(new_count);
        for i in 0..count {
            unsafe {
                let sample = old_samples.add(i).read();
                pointers.add(i).write(Box::into_raw(Box::new(sample)));
            }
        }

        // If it's shrinking.
        if new_count < old_count {
            for i in new_count..old_count {
                unsafe {
                    old_samples.add(i).drop_in_place();
                }
            }
        }
    }

    // Allocate new samples for additional slots
    if new_count > old_count {
        let default_sample = T::from_key(&T::Key::default());
        for i in old_count..new_count {
            let pointer = Box::into_raw(Box::new(default_sample.clone()));
            unsafe {
                pointers.add(i).write(pointer);
            }
        }
    }
}

/// Free previously allocated samples.
///
/// Release any memory allocated by
/// [`serdata_ops::to_sample`][`crate::internal::ffi::serdata_ops::to_sample`].
///
/// ## Safety
/// `pointers` must be non-null and must point to a valid pointer that was allocated
/// via a `Box<T>`.
pub unsafe extern "C" fn free_samples<T>(
    _sertype: *const cyclonedds_sys::ddsi_sertype,
    pointers: *mut *mut std::ffi::c_void,
    count: usize,
    operation: cyclonedds_sys::dds_free_op_t,
) where
    T: crate::Topicable,
{
    let free_all = operation & cyclonedds_sys::DDS_FREE_ALL_BIT != 0;
    let free_contents = operation & cyclonedds_sys::DDS_FREE_CONTENTS_BIT != 0;
    let samples = pointers as *mut *mut T;

    if free_all {
        for i in 0..count {
            let sample = unsafe { Box::from_raw(*samples.add(i)) };
            drop(sample);
        }
    } else if free_contents {
        for i in 0..count {
            unsafe {
                (*samples.add(i)).drop_in_place();
            }
        }
    }
}

/// Compares two [`Sertype`] instances for equality.
///
/// # Safety
/// - The `lhs` and `rhs` must point to `Sertype`s previously constructed by the Rust API.
/// - The `type_name` field of the pointers must be a valid null-terminated string.
pub unsafe extern "C" fn equal<T>(
    lhs: *const cyclonedds_sys::ddsi_sertype,
    rhs: *const cyclonedds_sys::ddsi_sertype,
) -> bool
where
    T: crate::Topicable,
{
    let lhs = std::mem::ManuallyDrop::new(unsafe { Box::from_raw(lhs as *mut Sertype<T>) });
    let rhs = std::mem::ManuallyDrop::new(unsafe { Box::from_raw(rhs as *mut Sertype<T>) });

    // Also base this on the type support identifier?
    unsafe { CStr::from_ptr(lhs.inner.type_name) == CStr::from_ptr(rhs.inner.type_name) }
}

/// Compute a hash for a DDS data type.
///
/// # Safety
/// The provided `sertype` must be a valid sertype created through [Sertype::new].
pub unsafe extern "C" fn hash<T>(sertype: *const cyclonedds_sys::ddsi_sertype) -> u32
where
    T: crate::Topicable,
{
    let sertype = std::mem::ManuallyDrop::new(unsafe { Box::from_raw(sertype as *mut Sertype<T>) });

    let name = unsafe { CStr::from_ptr(sertype.inner.type_name) };
    let type_size = std::mem::size_of::<T>();

    // Prepare the 32-bit hash by running the default hasher which produces a
    // 64-bit output and then combining the high and low ends of the hash via
    // xor to produce a 32-bit output.
    let mut hasher = std::hash::DefaultHasher::new();
    name.hash(&mut hasher);
    type_size.hash(&mut hasher);
    let hash: u64 = hasher.finish();
    let hash: u32 = (hash ^ (hash >> 32)) as u32;

    hash
}

/// TODO Unimplemented
/// # Safety
pub unsafe extern "C" fn type_id<T>(
    sertype: *const cyclonedds_sys::ddsi_sertype,
    kind: cyclonedds_sys::ddsi_typeid_kind_t,
) -> *mut cyclonedds_sys::ddsi_typeid
where
    T: crate::Topicable,
{
    let args = (sertype, kind);
    todo!(
        "sertype_ops::type_id<{}>({args:?})",
        std::any::type_name::<T>()
    );
}

/// TODO Unimplemented
/// # Safety
pub unsafe extern "C" fn type_map<T>(
    sertype: *const cyclonedds_sys::ddsi_sertype,
) -> *mut cyclonedds_sys::ddsi_typemap
where
    T: crate::Topicable,
{
    let args = sertype;
    todo!(
        "sertype_ops::type_map<{}>({args:?})",
        std::any::type_name::<T>()
    )
}

/// TODO Unimplemented
/// # Safety
pub unsafe extern "C" fn type_info<T>(
    sertype: *const cyclonedds_sys::ddsi_sertype,
) -> *mut cyclonedds_sys::ddsi_typeinfo
where
    T: crate::Topicable,
{
    let args = sertype;
    todo!(
        "sertype_ops::type_info<{}>({args:?})",
        std::any::type_name::<T>()
    );
}

/// TODO Unimplemented
/// # Safety
pub unsafe extern "C" fn derive_sertype<T>(
    sertype: *const cyclonedds_sys::ddsi_sertype,
    data_representation: cyclonedds_sys::dds_data_representation_id_t,
    type_consistency_enforcement_qos: cyclonedds_sys::dds_type_consistency_enforcement_qospolicy,
) -> *mut cyclonedds_sys::ddsi_sertype
where
    T: crate::Topicable,
{
    let args = (
        sertype,
        data_representation,
        type_consistency_enforcement_qos,
    );
    todo!(
        "sertype_ops::derive_sertype<{}>({args:?})",
        std::any::type_name::<T>()
    )
}

/// TODO Unimplemented
/// # Safety
pub unsafe extern "C" fn get_serialized_size<T>(
    sertype: *const cyclonedds_sys::ddsi_sertype,
    serdata_kind: cyclonedds_sys::ddsi_serdata_kind,
    sample: *const std::ffi::c_void,
    size: *mut usize,
    encoding_identifier: *mut u16,
) -> i32
where
    T: crate::Topicable,
{
    let args = (sertype, serdata_kind, sample, size, encoding_identifier);
    todo!(
        "sertype_ops::get_serialized_size<{}>({args:?})",
        std::any::type_name::<T>()
    )
}

/// TODO Unimplemented
/// # Safety
pub unsafe extern "C" fn serialize_into<T>(
    sertype: *const cyclonedds_sys::ddsi_sertype,
    serdata_kind: cyclonedds_sys::ddsi_serdata_kind,
    sample: *const std::ffi::c_void,
    destination_buffer: *mut std::ffi::c_void,
    destination_buffer_length: usize,
) -> bool
where
    T: crate::Topicable,
{
    let args = (
        sertype,
        serdata_kind,
        sample,
        destination_buffer,
        destination_buffer_length,
    );
    todo!(
        "sertype_ops::serialize_into<{}>({args:?})",
        std::any::type_name::<T>()
    )
}
