//! Operations that allow Cyclone to interact with Rust allocated data structures.
//! These are threaded through the [`Serdata`] type.

use std::io::Write;

use crate::cdr_bounds::{CdrBounds, CdrSize};
use crate::internal::key_hash::KeyHash;
use crate::internal::serdata::Serdata;
use crate::internal::sertype::Sertype;
use crate::sample::SampleOrKeyInner as SampleOrKey;

/// The size of the RTPS header in bytes.
pub const DDSI_RTPS_HEADER_SIZE: usize = 4;

/// Compares the keys of two [`Serdata`] instances for equality.
///
/// ## Safety
/// The `lhs` and `rhs` must be non-null pointers to fully-initialized [`Serdata`].
pub unsafe extern "C" fn eqkey<T>(
    lhs: *const cyclonedds_sys::ddsi_serdata,
    rhs: *const cyclonedds_sys::ddsi_serdata,
) -> bool
where
    T: crate::Topicable,
{
    let mut lhs = std::mem::ManuallyDrop::new(unsafe { Box::from_raw(lhs as *mut Serdata<T>) });
    let mut rhs = std::mem::ManuallyDrop::new(unsafe { Box::from_raw(rhs as *mut Serdata<T>) });

    lhs.key() == rhs.key()
}

/// Returns the serialized size (in bytes) of the sample contained in the given
/// `Serdata` + the size of the DDSI RTPS header.
///
/// ## Safety
/// The `serdata`  must be a non-null pointer to a fully-initialized [`Serdata`].
pub unsafe extern "C" fn get_size<T>(serdata: *const cyclonedds_sys::ddsi_serdata) -> u32
where
    T: crate::Topicable,
{
    let mut serdata =
        std::mem::ManuallyDrop::new(unsafe { Box::from_raw(serdata as *mut Serdata<T>) });

    serdata
        .serialized()
        .expect("unable to serialize data") // TODO pass this back out somehow?
        .len() as u32
}

#[inline]
unsafe fn ddsi_rmsg_payloadoff(m: *const cyclonedds_sys::ddsi_rmsg, o: usize) -> *const u8 {
    // Skip the rmsg header, then add offset
    unsafe { (m as *const u8).add(std::mem::size_of::<cyclonedds_sys::ddsi_rmsg>() + o) }
}

#[inline]
fn ddsi_rdata_payloadoff(rdata: &cyclonedds_sys::ddsi_rdata) -> usize {
    rdata.payload_zoff as usize
}

unsafe fn copy_from_fragment(
    buffer: &mut [u8],
    mut fragment_chain: *const cyclonedds_sys::ddsi_rdata,
    size: usize,
) -> Result<(), ()> {
    let mut offset: usize = 0;

    if fragment_chain.is_null()
        || unsafe { *fragment_chain }.min != 0
        || (unsafe { *fragment_chain }.maxp1 as usize) < offset
    {
        return Err(());
    }

    let mut cursor = buffer.as_mut_ptr();

    while !fragment_chain.is_null() {
        let fragment = unsafe { &*fragment_chain };
        let maxp1 = fragment.maxp1 as usize;
        let min = fragment.min as usize;
        if maxp1 > offset {
            let payload =
                unsafe { ddsi_rmsg_payloadoff(fragment.rmsg, ddsi_rdata_payloadoff(fragment)) };
            let src = unsafe { payload.add(offset - min) };
            let number_of_bytes = maxp1 - offset;

            if offset + number_of_bytes > size {
                return Err(());
            }

            cursor = unsafe {
                std::ptr::copy_nonoverlapping(src, cursor, number_of_bytes);
                cursor.add(number_of_bytes)
            };
            offset = maxp1;
        }

        fragment_chain = fragment.nextfrag;
    }

    if offset != size { Err(()) } else { Ok(()) }
}

/// TODO Unimplemented
/// ## Safety
pub unsafe extern "C" fn from_ser<T>(
    sertype: *const cyclonedds_sys::ddsi_sertype,
    kind: cyclonedds_sys::ddsi_serdata_kind,
    fragment_chain: *const cyclonedds_sys::ddsi_rdata,
    size: usize,
) -> *mut cyclonedds_sys::ddsi_serdata
where
    T: crate::Topicable,
{
    let sertype = std::mem::ManuallyDrop::new(unsafe { Box::from_raw(sertype as *mut Sertype<T>) });

    if let Ok(kind) = crate::internal::serdata::Kind::try_from(kind) {
        let mut buffer = vec![0; size];

        if unsafe { copy_from_fragment(&mut buffer, fragment_chain, size) }.is_ok() {
            match kind {
                crate::internal::serdata::Kind::Key => {
                    if let Ok((key, _)) = cdr_encoding::from_bytes::<T::Key, byteorder::NativeEndian>(
                        &buffer[DDSI_RTPS_HEADER_SIZE..],
                    ) {
                        let key = SampleOrKey::new_key(key);
                        let serdata = crate::internal::serdata::Serdata::new(&sertype, key);
                        Box::into_raw(serdata) as *mut _
                    } else {
                        std::ptr::null_mut()
                    }
                }
                crate::internal::serdata::Kind::Data => {
                    if let Ok((data, _)) = cdr_encoding::from_bytes::<T, byteorder::NativeEndian>(
                        &buffer[DDSI_RTPS_HEADER_SIZE..],
                    ) {
                        let sample = SampleOrKey::new_sample(data);
                        let serdata = crate::internal::serdata::Serdata::new(&sertype, sample);
                        Box::into_raw(serdata) as *mut _
                    } else {
                        std::ptr::null_mut()
                    }
                }
            }
        } else {
            std::ptr::null_mut()
        }
    } else {
        std::ptr::null_mut()
    }
}

/// Construct a [`Serdata`] from a serialized `iovec` and return it as a raw pointer.
///
/// ## Safety
/// - `serdata` must be a non-null pointer to a fully-initialized [`Serdata`].
/// - `containers` must point to a valid contiguous array of `iovec` structures of length
///   `containers_len`. Each `iov_base` must be a valid pointer to `iov_len` bytes of readable
///   memory.
/// - `size` must be less than or equal to the total number of bytes described by the `iovec` array,
///   or else out-of-bounds memory will be accessed when copying data.
/// - `T` must match the expected deserialized type, and the deserialization must not violate any
///   invariants assumed by the type (e.g., panic during construction or interior mutability
///   issues).
pub unsafe extern "C" fn from_ser_iov<T>(
    sertype: *const cyclonedds_sys::ddsi_sertype,
    kind: cyclonedds_sys::ddsi_serdata_kind,
    containers_len: cyclonedds_sys::ddsrt_msg_iovlen_t,
    containers: *const cyclonedds_sys::iovec,
    size: usize,
) -> *mut cyclonedds_sys::ddsi_serdata
where
    T: crate::Topicable,
{
    let sertype = std::mem::ManuallyDrop::new(unsafe { Box::from_raw(sertype as *mut Sertype<T>) });

    if let Ok(kind) = crate::internal::serdata::Kind::try_from(kind) {
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
        match kind {
            crate::internal::serdata::Kind::Key => {
                if let Ok((key, _length)) = cdr_encoding::from_bytes::<_, byteorder::NativeEndian>(
                    &serialized_sample[DDSI_RTPS_HEADER_SIZE..],
                ) {
                    let key = SampleOrKey::new_key(key);
                    let serdata = Serdata::new(sertype.as_ref(), key);
                    serdata.serialized_sample.set(serialized_sample).unwrap();
                    Box::into_raw(serdata) as *mut _
                } else {
                    std::ptr::null_mut()
                }
            }
            crate::internal::serdata::Kind::Data => {
                if let Ok((sample, _length)) = cdr_encoding::from_bytes::<_, byteorder::NativeEndian>(
                    &serialized_sample[DDSI_RTPS_HEADER_SIZE..],
                ) {
                    let sample = SampleOrKey::new_sample(sample);
                    let serdata = Serdata::new(sertype.as_ref(), sample);
                    serdata.serialized_sample.set(serialized_sample).unwrap();
                    Box::into_raw(serdata) as *mut _
                } else {
                    std::ptr::null_mut()
                }
            }
        }
    } else {
        std::ptr::null_mut()
    }
}

/// TODO Unimplemented
/// ## Safety
pub unsafe extern "C" fn from_keyhash<T>(
    sertype: *const cyclonedds_sys::ddsi_sertype,
    keyhash: *const cyclonedds_sys::ddsi_keyhash,
) -> *mut cyclonedds_sys::ddsi_serdata
where
    T: crate::Topicable,
{
    let sertype = std::mem::ManuallyDrop::new(unsafe { Box::from_raw(sertype as *mut Sertype<T>) });
    let keyhash = unsafe { &*keyhash };

    let max_possible_serialized_size = T::Key::max_serialized_cdr_size();

    let force_md5 = T::FORCE_MD5_KEYHASH;
    if force_md5 || max_possible_serialized_size > CdrSize::Bounded(16) {
        // The key hash is based on MD5 and so can't be reconstructed into a key.
        std::ptr::null_mut()
    } else {
        // The key hash is just the big-endian CDR serialized form of the key.
        if let Ok((key, _)) = cdr_encoding::from_bytes::<_, byteorder::BigEndian>(&keyhash.value) {
            let serdata = Serdata::new(&sertype, SampleOrKey::<T>::new_key(key));
            Box::into_raw(serdata) as *mut _
        } else {
            std::ptr::null_mut()
        }
    }
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
    T: crate::Topicable,
{
    match crate::internal::serdata::Kind::try_from(kind) {
        Ok(crate::internal::serdata::Kind::Data) => {
            let sample = unsafe { &*(sample as *const T) };

            let sertype =
                std::mem::ManuallyDrop::new(unsafe { Box::from_raw(sertype as *mut Sertype<T>) });

            let sample = SampleOrKey::new_sample(sample.clone());
            let serdata = Serdata::new(sertype.as_ref(), sample);

            Box::into_raw(serdata) as *mut _
        }
        Ok(crate::internal::serdata::Kind::Key) => {
            let key = unsafe { &*(sample as *const T::Key) };

            let sertype =
                std::mem::ManuallyDrop::new(unsafe { Box::from_raw(sertype as *mut Sertype<T>) });

            let key = SampleOrKey::new_key(key.clone());
            let serdata = Serdata::new(sertype.as_ref(), key);

            Box::into_raw(serdata) as *mut _
        }
        _ => std::ptr::null_mut(),
    }
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
    T: crate::Topicable,
{
    let mut serdata =
        std::mem::ManuallyDrop::new(unsafe { Box::from_raw(serdata as *mut Serdata<T>) });
    let container = unsafe { &mut *container };

    let serialized = serdata.serialized_with_size(size);

    container.iov_base = serialized[offset..].as_ptr() as *mut _;
    container.iov_len = serialized.len();
    unsafe { cyclonedds_sys::ddsi_serdata_ref(&serdata.inner) }
}

/// Relinquish the reference handed out by [`to_ser_ref`].
///
/// This decrements the `serdata` reference counter.
///
/// ## Safety
/// - `serdata`  must be a non-null pointer to a fully-initialized [`Serdata`].
pub unsafe extern "C" fn to_ser_unref<T>(
    serdata: *mut cyclonedds_sys::ddsi_serdata,
    _: *const cyclonedds_sys::iovec,
) where
    T: crate::Topicable,
{
    let mut serdata =
        std::mem::ManuallyDrop::new(unsafe { Box::from_raw(serdata as *mut Serdata<T>) });

    crate::internal::ffi::ddsi_serdata_unref(&mut serdata.inner);
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
    T: crate::Topicable,
{
    let mut serdata =
        std::mem::ManuallyDrop::new(unsafe { Box::from_raw(serdata as *mut Serdata<T>) });

    match serdata.kind() {
        crate::internal::serdata::Kind::Key => {
            let data = serdata.key();
            let key = sample as *mut T::Key;
            unsafe {
                key.write(data.clone());
            }
            true
        }
        crate::internal::serdata::Kind::Data => {
            let data = serdata.sample();
            let sample = sample as *mut T;
            unsafe {
                sample.write(data.clone());
            }
            true
        }
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
    T: crate::Topicable,
{
    let serdata = std::mem::ManuallyDrop::new(unsafe { Box::from_raw(serdata as *mut Serdata<T>) });

    let sertype = std::mem::ManuallyDrop::new(unsafe {
        Box::from_raw(serdata.inner.type_ as *mut Sertype<T>)
    });

    let mut untyped_serdata = Serdata::new(
        sertype.as_ref(),
        SampleOrKey::new_key(serdata.sample.as_ref().key().clone()),
    );
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
    assert!(
        !sample.is_null(),
        "untyped_to_sample::<{}>",
        std::any::type_name::<T>()
    );

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
pub unsafe extern "C" fn free<T>(serdata: *mut cyclonedds_sys::ddsi_serdata)
where
    T: crate::Topicable,
{
    let serdata = unsafe { Box::from_raw(serdata as *mut Serdata<T>) };

    drop(serdata);
}

/// Writes the debug representation of a [`Serdata`] into the provided buffer.
///
/// This output relies on `T`’s [`std::fmt::Debug`] implementation to display
/// the sample held by the [`Serdata`]. At most `length` bytes are written. If
/// the formatted string is longer, the result is truncated. The buffer is
/// always null-terminated.
///
/// the return value is the number of bytes written to the buffer, **excluding**
/// the null terminator.
///
/// ## Safety
/// `serdata` must point to a valid previously allocated [`Serdata`].
/// TODO
pub unsafe extern "C" fn print<T>(
    _sertype: *const cyclonedds_sys::ddsi_sertype,
    serdata: *const cyclonedds_sys::ddsi_serdata,
    buffer: *mut i8,
    length: usize,
) -> usize
where
    T: crate::Topicable,
{
    let serdata = std::mem::ManuallyDrop::new(unsafe { Box::from_raw(serdata as *mut Serdata<T>) });

    let buffer = unsafe { std::slice::from_raw_parts_mut(buffer as *mut u8, length) };
    let mut cursor = std::io::Cursor::new(&mut *buffer);

    // Ignore a potential error when writing.
    let _ = write!(cursor, "{:#?}", &*serdata);
    // Ensure that whatever was written is null-terminated.
    let written = (cursor.position() as usize).min(length.saturating_sub(1));
    buffer[written] = 0;
    written
}

/// TODO Unimplemented
/// ## Safety
pub unsafe extern "C" fn get_keyhash<T>(
    serdata: *const cyclonedds_sys::ddsi_serdata,
    keyhash: *mut cyclonedds_sys::ddsi_keyhash,
    force_md5: bool,
) where
    T: crate::Topicable,
{
    let mut serdata =
        std::mem::ManuallyDrop::new(unsafe { Box::from_raw(serdata as *mut Serdata<T>) });
    let keyhash = unsafe { &mut *keyhash };

    if let Some(serdata_keyhash) = KeyHash::from_key::<T>(serdata.key(), force_md5) {
        keyhash.value.copy_from_slice(&serdata_keyhash.0);
    }
}

/// TODO Unimplemented
/// ## Safety
pub unsafe extern "C" fn from_loaned_sample<T>(
    sertype: *const cyclonedds_sys::ddsi_sertype,
    a: u32,
    b: *const i8,
    c: *mut cyclonedds_sys::dds_loaned_sample,
    d: bool,
) -> *mut cyclonedds_sys::ddsi_serdata
where
    T: crate::Topicable,
{
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
) -> *mut cyclonedds_sys::ddsi_serdata
where
    T: crate::Topicable,
{
    let args = (sertype, loan);
    todo!(
        "serdata_ops::from_psmx<{}>({args:?})",
        std::any::type_name::<T>()
    )
}
