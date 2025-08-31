//! Operations that allow Cyclone to interact with Rust allocated data structures.
//! These are threaded through the [`Serdata`] type.

use crate::internal::serdata::Serdata;
use crate::internal::sertype::Sertype;

/// The size of the RTPS header in bytes.
pub const DDSI_RTPS_HEADER_SIZE: usize = 4;

/// Compares the keys of two [`Serdata`] instances for equality.
///
/// ## Safety
/// The `lhs` and `rhs` must be non-null pointers to fully-initialized [`Serdata`].
pub unsafe extern "C" fn eqkey<T>(
    lhs: *const cyclonedds_sys::ddsi_serdata,
    rhs: *const cyclonedds_sys::ddsi_serdata,
) -> bool {
    let lhs = std::mem::ManuallyDrop::new(unsafe { Box::from_raw(lhs as *mut Serdata<T>) });
    let rhs = std::mem::ManuallyDrop::new(unsafe { Box::from_raw(rhs as *mut Serdata<T>) });

    lhs.key == rhs.key
}

/// Returns the serialized size (in bytes) of the sample contained in the given `Serdata`.
///
/// If no sample is present, returns `0`.
///
/// ## Safety
/// The `serdata`  must be a non-null pointer to a fully-initialized [`Serdata`].
pub unsafe extern "C" fn get_size<T>(serdata: *const cyclonedds_sys::ddsi_serdata) -> u32
where
    T: serde::ser::Serialize + std::clone::Clone,
{
    let mut serdata =
        std::mem::ManuallyDrop::new(unsafe { Box::from_raw(serdata as *mut Serdata<T>) });

    if let Some(sample) = serdata.sample() {
        cdr_encoding::to_vec::<_, byteorder::LittleEndian>(sample)
            .map_or(0, |bytes| bytes.len() as u32)
    } else {
        0
    }
}

/// TODO Unimplemented
/// ## Safety
pub unsafe extern "C" fn from_ser<T>(
    serdata: *const cyclonedds_sys::ddsi_sertype,
    a: u32,
    b: *const cyclonedds_sys::ddsi_rdata,
    c: usize,
) -> *mut cyclonedds_sys::ddsi_serdata {
    let args = (serdata, a, b, c);
    todo!(
        "serdata_ops::from_ser<{}>({args:?})",
        std::any::type_name::<T>()
    )
}

/// Construct a [`Serdata`] from a serialized `iovec` and return it as a raw pointer.
///
/// ## Safety
/// - `serdata` must be a non-null pointer to a fully-initialized [`Serdata`].
/// - `containers` must point to a valid contiguous array of `iovec` structures of length `containers_len`.
///   Each `iov_base` must be a valid pointer to `iov_len` bytes of readable memory.
/// - `size` must be less than or equal to the total number of bytes described by the `iovec` array,
///   or else out-of-bounds memory will be accessed when copying data.
/// - `T` must match the expected deserialized type, and the deserialization must not violate any invariants
///   assumed by the type (e.g., panic during construction or interior mutability issues).
pub unsafe extern "C" fn from_ser_iov<T>(
    sertype: *const cyclonedds_sys::ddsi_sertype,
    kind: cyclonedds_sys::ddsi_serdata_kind,
    containers_len: cyclonedds_sys::ddsrt_msg_iovlen_t,
    containers: *const cyclonedds_sys::iovec,
    size: usize,
) -> *mut cyclonedds_sys::ddsi_serdata
where
    T: std::clone::Clone + serde::de::DeserializeOwned,
{
    let sertype = std::mem::ManuallyDrop::new(unsafe { Box::from_raw(sertype as *mut Sertype<T>) });

    if let Ok(kind) = crate::internal::serdata::Kind::try_from(kind) {
        let mut serdata = Serdata::new(sertype.as_ref(), kind);

        let mut serialized_sample: Vec<u8> = Vec::with_capacity(size);
        // NOTE: `ddsrt_msg_iovlen_t` might not be a usize on all platforms and so the clippy lint
        // warning about unnecessary casts should be suppressed.
        #[allow(clippy::unnecessary_cast)]
        let containers = unsafe { std::slice::from_raw_parts(containers, containers_len as usize) };

        let mut offset = 0;
        for container in containers {
            let container_len = if container.iov_len + offset > size {
                size - offset
            } else {
                container.iov_len
            };

            let container = unsafe {
                std::slice::from_raw_parts(container.iov_base as *const u8, container.iov_len)
            };
            serialized_sample.extend_from_slice(container);
            offset += container_len;
        }
        if serdata.sample.is_none()
            && let Ok((sample, _length)) =
                cdr_encoding::from_bytes::<_, byteorder::LittleEndian>(&serialized_sample)
        {
            serdata.sample.replace(std::sync::Arc::new(sample));
        }
        serdata.serialized_sample = Some(serialized_sample);

        Box::into_raw(serdata) as *mut _
    } else {
        std::ptr::null_mut()
    }
}

/// TODO Unimplemented
/// ## Safety
pub unsafe extern "C" fn from_keyhash<T>(
    sertype: *const cyclonedds_sys::ddsi_sertype,
    a: *const cyclonedds_sys::ddsi_keyhash,
) -> *mut cyclonedds_sys::ddsi_serdata {
    let args = (sertype, a);
    todo!(
        "serdata_ops::from_keyhash<{}>({args:?})",
        std::any::type_name::<T>()
    )
}

/// Constructs a [`Serdata`] from a sample pointer, given a serialization kind.
///
/// ## Safety
/// `sertype` must be a valid, non-null pointer to a heap-allocated [`Sertype`].
pub unsafe extern "C" fn from_sample<T>(
    sertype: *const cyclonedds_sys::ddsi_sertype,
    kind: cyclonedds_sys::ddsi_serdata_kind,
    sample: *const std::ffi::c_void,
) -> *mut cyclonedds_sys::ddsi_serdata
where
    T: std::clone::Clone,
{
    match crate::internal::serdata::Kind::try_from(kind) {
        Ok(kind @ crate::internal::serdata::Kind::Data) => {
            let sample = unsafe { &*(sample as *const T) };

            let sertype =
                std::mem::ManuallyDrop::new(unsafe { Box::from_raw(sertype as *mut Sertype<T>) });

            let mut serdata = Serdata::new(sertype.as_ref(), kind);

            // TODO fix this. A clone bound shouldn't be needed.
            serdata.sample.replace(std::sync::Arc::new(sample.clone()));
            Box::into_raw(serdata) as *mut _
        }
        Ok(kind @ crate::internal::serdata::Kind::Key) => todo!("{kind:?}"),
        _ => std::ptr::null_mut(),
    }
    // if kind is key then `get_serialized_size` with key_mode::unsorted must be true
    // if kind is not key then `get_serialized_size` with key_mode::not_key must be true

    // add 4 to the header size
    // tell the serdata to resize to the size
    //
    //
    // serialize the data into the serdata
    // set the key hash
    // set the T
    // populate the hash
}

/// TODO Unimplemented
/// ## Safety
pub unsafe extern "C" fn to_ser<T>(
    serdata: *const cyclonedds_sys::ddsi_serdata,
    a: usize,
    b: usize,
    c: *mut std::ffi::c_void,
) {
    let args = (serdata, a, b, c);
    todo!(
        "serdata_ops::to_ser<{}>({args:?})",
        std::any::type_name::<T>()
    )
}

/// Write the serialized bytes of a sample to the provided [`cyclonedds_sys::iovec`].
///
/// This increments the `serdata` reference counter.
///
/// ## Safety
/// - `serdata`  must be a non-null pointer to a fully-initialized [`Serdata`].
/// - `container` must be a non-null pointer to an [`cyclonedds_sys::iovec`].
pub unsafe extern "C" fn to_ser_ref<T>(
    serdata: *const cyclonedds_sys::ddsi_serdata,
    offset: usize,
    size: usize,
    container: *mut cyclonedds_sys::iovec,
) -> *mut cyclonedds_sys::ddsi_serdata
where
    T: std::clone::Clone + serde::ser::Serialize,
{
    let mut serdata =
        std::mem::ManuallyDrop::new(unsafe { Box::from_raw(serdata as *mut Serdata<T>) });
    let container = unsafe { &mut *container };

    if serdata.serialized_sample.is_none()
        && let Some(sample) = serdata.sample()
    {
        serdata.serialized_sample = cdr_encoding::to_vec::<_, byteorder::LittleEndian>(sample).ok();
    }

    if let Some(serialized_sample) = &serdata.serialized_sample {
        container.iov_base = (serialized_sample[offset..(offset + size)]).as_ptr() as *mut _;
        container.iov_len = size;
        unsafe { cyclonedds_sys::ddsi_serdata_ref(&serdata.inner) }
    } else {
        std::ptr::null_mut()
    }
}

/// Relinquish the reference handed out by [`to_ser_ref`].
///
/// This decrements the `serdata` reference counter.
///
/// ## Safety
/// - `serdata`  must be a non-null pointer to a fully-initialized [`Serdata`].
///
pub unsafe extern "C" fn to_ser_unref<T>(
    serdata: *mut cyclonedds_sys::ddsi_serdata,
    _: *const cyclonedds_sys::iovec,
) {
    let mut serdata =
        std::mem::ManuallyDrop::new(unsafe { Box::from_raw(serdata as *mut Serdata<T>) });

    unsafe {
        cyclonedds_sys::ddsi_serdata_unref(&mut serdata.inner);
    }
}

/// Copies a (deserialized) sample from a [`Serdata`] into a provided `sample` pointer.
///
/// Returns `true` if a sample was present and copied, `false` otherwise.
///
/// ## Safety
/// - `sertype` must be a valid, non-null pointer to a heap-allocated [`Sertype`].
/// - `sample` must be a valid, non-null pointer that can hold a value of size `T`.
pub unsafe extern "C" fn to_sample<T>(
    serdata: *const cyclonedds_sys::ddsi_serdata,
    sample: *mut std::ffi::c_void,
    _buffer: *mut *mut std::ffi::c_void,
    _buffer_limit: *mut std::ffi::c_void,
) -> bool
where
    T: std::clone::Clone,
{
    let mut serdata =
        std::mem::ManuallyDrop::new(unsafe { Box::from_raw(serdata as *mut Serdata<T>) });

    if let Some(data) = serdata.sample() {
        let sample = sample as *mut T;
        unsafe {
            sample.write(data.clone());
        }

        true
    } else {
        false
    }
}

/// Create an untyped sertype based on provided typed [`Serdata`]
///
/// ## Safety
/// - `sertype` must be a valid, non-null pointer to a heap-allocated [`Sertype`].
pub unsafe extern "C" fn to_untyped<T>(
    serdata: *const cyclonedds_sys::ddsi_serdata,
) -> *mut cyclonedds_sys::ddsi_serdata
where
    T: std::clone::Clone,
{
    let serdata = std::mem::ManuallyDrop::new(unsafe { Box::from_raw(serdata as *mut Serdata<T>) });

    let sertype = std::mem::ManuallyDrop::new(unsafe {
        Box::from_raw(serdata.inner.type_ as *mut Sertype<T>)
    });

    let mut untyped_serdata = Serdata::new(sertype.as_ref(), crate::internal::serdata::Kind::Key);
    untyped_serdata.inner.type_ = std::ptr::null_mut();

    Box::into_raw(untyped_serdata) as *mut _
}

/// TODO Unimplemented
/// ## Safety
pub unsafe extern "C" fn untyped_to_sample<T>(
    _sertype: *const cyclonedds_sys::ddsi_sertype,
    _serdata: *const cyclonedds_sys::ddsi_serdata,
    sample: *mut std::ffi::c_void,
    _buffer: *mut *mut std::ffi::c_void,
    _buffer_limit: *mut std::ffi::c_void,
) -> bool {
    assert!(!sample.is_null());

    // let sample = unsafe { &mut *(sample as *mut std::mem::ManuallyDrop<T>) };
    // unsafe {
    //     std::ptr::drop_in_place(sample);
    // }

    // auto d = static_cast<const ddscxx_serdata<T>*>(dcmn);
    // T* ptr = static_cast<T*>(sample);

    // return deserialize_sample_from_buffer(d->data(), d->size(), *ptr, SDK_KEY);

    false
}

/// Deallocate a [`Serdata`].
///
/// ## Safety
/// `serdata` must point to a valid previously allocated [`Serdata`].
/// TODO
pub unsafe extern "C" fn free<T>(serdata: *mut cyclonedds_sys::ddsi_serdata) {
    let serdata = unsafe { Box::from_raw(serdata as *mut Serdata<T>) };

    drop(serdata);
}

/// TODO Unimplemented
/// ## Safety
pub unsafe extern "C" fn print<T>(
    sertype: *const cyclonedds_sys::ddsi_sertype,
    serdata: *const cyclonedds_sys::ddsi_serdata,
    a: *mut i8,
    b: usize,
) -> usize {
    let args = (sertype, serdata, a, b);
    todo!(
        "serdata_ops::print<{}>({args:?})",
        std::any::type_name::<T>()
    )
}

/// TODO Unimplemented
/// ## Safety
pub unsafe extern "C" fn get_keyhash<T>(
    serdata: *const cyclonedds_sys::ddsi_serdata,
    a: *mut cyclonedds_sys::ddsi_keyhash,
    b: bool,
) {
    let args = (serdata, a, b);
    todo!(
        "serdata_ops::get_keyhash<{}>({args:?})",
        std::any::type_name::<T>()
    )
}

/// TODO Unimplemented
/// ## Safety
pub unsafe extern "C" fn from_loaned_sample<T>(
    sertype: *const cyclonedds_sys::ddsi_sertype,
    a: u32,
    b: *const i8,
    c: *mut cyclonedds_sys::dds_loaned_sample,
    d: bool,
) -> *mut cyclonedds_sys::ddsi_serdata {
    let args = (sertype, a, b, c, d);
    todo!(
        "serdata_ops::from_loaned_sample<{}>({args:?})",
        std::any::type_name::<T>()
    )
}

/// TODO Unimplemented
/// ## Safety
pub unsafe extern "C" fn from_psmx<T>(
    sertype: *const cyclonedds_sys::ddsi_sertype,
    loan: *mut cyclonedds_sys::dds_loaned_sample,
) -> *mut cyclonedds_sys::ddsi_serdata {
    let args = (sertype, loan);
    todo!(
        "serdata_ops::from_psmx<{}>({args:?})",
        std::any::type_name::<T>()
    )
}
