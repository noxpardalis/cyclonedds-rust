//! Operations that allow Cyclone to interact with Rust allocated data structures.
//! These are threaded through the [`Sertype`] type.

use crate::internal::sertype::Sertype;
use crate::internal::traits::Hash32;
use std::ffi::CStr;

/// A version to allow Cyclone DDS to ensure backwards compatibility if breaking changes to the
/// sertype API are introduced.
pub const VERSION: cyclonedds_sys::ddsi_sertype_v0_t = Some(cyclonedds_sys::ddsi_sertype_v0);

/// Arbitrary metadata that was originally used for backwards compatibility as part of the
/// `sertopic` -> `sertype` changeover.
pub const ARG: *mut std::ffi::c_void = std::ptr::null_mut();

/// A flag specifying that the data representation in use corresponds to XCDR1.
#[allow(dead_code)]
pub const DATA_REPRESENTATION_XCDR1: u32 = cyclonedds_sys::DDS_DATA_REPRESENTATION_XCDR1;
/// A flag specifying that the data representation in use corresponds to XCDR2.
#[allow(dead_code)]
pub const DATA_REPRESENTATION_XCDR2: u32 = cyclonedds_sys::DDS_DATA_REPRESENTATION_XCDR2;

/// Free a [`Sertype`] created on the Rust side of the FFI.
///
/// ## Safety
/// The provided `sertype` pointer must be from the pointer embedded in a [`Sertype`] that was
/// created via [`Sertype::new`].
pub unsafe extern "C" fn free<T>(_sertype: *mut cyclonedds_sys::ddsi_sertype) {
    // let mut sertype = std::mem::ManuallyDrop::new(unsafe { Box::from_raw(sertype as *mut Sertype<T>) });
    // unsafe {
    //     cyclonedds_sys::ddsi_sertype_fini(&mut sertype.inner);
    // }
    // drop(sertype);
}

/// Note this function is a no-op.
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
    debug_assert_eq!(count, 1);
    let samples = samples as *mut T;

    let data = T::from_key(&T::Key::default());
    unsafe {
        std::ptr::write(samples, data);
    }
}

/// Realloc the sample buffer.
///
/// This makes a similar assumption as in the C++ wrapper regarding the caller of
/// this function: it is only ever invoked by `ddsi_sertype_alloc_sample`. This
/// guarantees that it is never used to reallocate an existing sample collection.
///
/// Therefore, `realloc_samples` is only used in the initial allocation path,
/// where `old_count` is always 0, `new_count` is always 1, and
/// `old_samples` is always null.
///
/// As a result, this function simply allocates a single default-initialized
/// instance of `T` using `Default` and returns it via the `pointers` array.
pub unsafe extern "C" fn realloc_samples<T>(
    pointers: *mut *mut ::std::ffi::c_void,
    _sertype: *const cyclonedds_sys::ddsi_sertype,
    old_samples: *mut ::std::ffi::c_void,
    old_count: usize,
    new_count: usize,
) where
    T: crate::Topicable,
{
    if new_count == 0 {
        return;
    }
    if new_count < old_count {
        return;
    }

    if old_samples.is_null() {
        for i in 0..new_count {
            let pointer: *mut T = Box::into_raw(Box::new(T::from_key(&T::Key::default())));
            unsafe {
                pointers.add(i).write(pointer as *mut _);
            }
        }
    } else {
        for i in 0..new_count {
            unsafe {
                // TODO confirm `...write(old_samples.add(1))` or should it
                // be `...write(old_samples)`.
                pointers.add(i).write(old_samples.add(1));
            }
        }
    }
}

/// Free previously allocated samples.
///
/// Release any memory allocated by [`serdata_ops::to_sample`][`crate::internal::ffi::serdata_ops::to_sample`].
///
/// ## Safety
/// `pointers` must be non-null and must point to a valid pointer that was allocated
/// via a `Box<T>`.
pub unsafe extern "C" fn free_samples<T>(
    _sertype: *const cyclonedds_sys::ddsi_sertype,
    pointers: *mut *mut ::std::ffi::c_void,
    count: usize,
    operation: cyclonedds_sys::dds_free_op_t,
) where
    T: crate::Topicable,
{
    debug_assert_eq!(count, 1);
    let pointer = unsafe { *pointers as *mut T };

    if operation & cyclonedds_sys::DDS_FREE_ALL_BIT != 0 {
        let data = unsafe { Box::from_raw(pointer) };
        drop(data);
    } else {
        assert!(operation & cyclonedds_sys::DDS_FREE_CONTENTS_BIT != 0);
        let data = T::from_key(&T::Key::default());
        unsafe {
            // TODO should I?: std::ptr::drop_in_place(pointer);
            eprintln!("pointer: {:p}", pointer);
            std::ptr::write(pointer, data);
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
) -> bool {
    let lhs = std::mem::ManuallyDrop::new(unsafe { Box::from_raw(lhs as *mut Sertype<T>) });
    let rhs = std::mem::ManuallyDrop::new(unsafe { Box::from_raw(rhs as *mut Sertype<T>) });

    // Also base this on the type support identifier?
    unsafe { CStr::from_ptr(lhs.inner.type_name) == CStr::from_ptr(rhs.inner.type_name) }
}

/// Compute a hash for a DDS data type.
///
/// # Safety
/// The provided `sertype` must be a valid sertype created through [Sertype::new].
pub unsafe extern "C" fn hash<T>(sertype: *const cyclonedds_sys::ddsi_sertype) -> u32 {
    let sertype = std::mem::ManuallyDrop::new(unsafe { Box::from_raw(sertype as *mut Sertype<T>) });

    let name = unsafe { CStr::from_ptr(sertype.inner.type_name) };
    let type_size = std::mem::size_of::<T>();

    (name, type_size).hash32()
}

/// TODO Unimplemented
/// # Safety
#[allow(dead_code)]
pub unsafe extern "C" fn type_id<T>(
    sertype: *const cyclonedds_sys::ddsi_sertype,
    kind: cyclonedds_sys::ddsi_typeid_kind_t,
) -> *mut cyclonedds_sys::ddsi_typeid {
    let args = (sertype, kind);
    todo!(
        "sertype_ops::type_id<{}>({args:?})",
        std::any::type_name::<T>()
    );
}

/// TODO Unimplemented
/// # Safety
#[allow(dead_code)]
pub unsafe extern "C" fn type_map<T>(
    sertype: *const cyclonedds_sys::ddsi_sertype,
) -> *mut cyclonedds_sys::ddsi_typemap {
    let args = sertype;
    todo!(
        "sertype_ops::type_map<{}>({args:?})",
        std::any::type_name::<T>()
    )
}

/// TODO Unimplemented
/// # Safety
#[allow(dead_code)]
pub unsafe extern "C" fn type_info<T>(
    sertype: *const cyclonedds_sys::ddsi_sertype,
) -> *mut cyclonedds_sys::ddsi_typeinfo {
    let args = sertype;
    todo!(
        "sertype_ops::type_info<{}>({args:?})",
        std::any::type_name::<T>()
    );
}

/// TODO Unimplemented
/// # Safety
#[allow(dead_code)]
pub unsafe extern "C" fn derive_sertype<T>(
    sertype: *const cyclonedds_sys::ddsi_sertype,
    data_representation: cyclonedds_sys::dds_data_representation_id_t,
    type_consistency_enforcement_qos: cyclonedds_sys::dds_type_consistency_enforcement_qospolicy,
) -> *mut cyclonedds_sys::ddsi_sertype
where
    T: std::clone::Clone,
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
) -> i32 {
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
) -> bool {
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
