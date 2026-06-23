use super::read_operation;
use crate::Result;
use crate::error::IntoError;

pub fn dds_peek_read_take<T, RO>(
    reader: cyclonedds_sys::dds_entity_t,
) -> Result<Vec<crate::sample::SampleOrKey<T>>>
where
    T: crate::builtin::private::BuiltInTopicType,
    RO: read_operation::ReadOperation,
{
    let mut samples = Vec::<crate::sample::SampleOrKey<T>>::new();

    let handle = Default::default();
    let mask = Default::default();
    let maxs = i32::MAX as u32;
    let len = usize::try_from(
        unsafe {
            RO::COLLECTOR(
                reader,
                maxs,
                handle,
                mask,
                Some(builtin_read_with_collector_callback::<T>),
                (&raw mut samples).cast(),
            )
        }
        .into_error()?,
    )
    .expect("len is a non-negative i32 and so always fits in a u32");

    assert_eq!(
        len,
        samples.len(),
        "number of built-in samples reported from the C side does not match the final number in the buffer"
    );

    Ok(samples)
}

pub fn dds_take<T>(
    reader: cyclonedds_sys::dds_entity_t,
) -> Result<Vec<crate::sample::SampleOrKey<T>>>
where
    T: crate::builtin::private::BuiltInTopicType,
{
    dds_peek_read_take::<T, read_operation::Take>(reader)
}

pub fn dds_read<T>(
    reader: cyclonedds_sys::dds_entity_t,
) -> Result<Vec<crate::sample::SampleOrKey<T>>>
where
    T: crate::builtin::private::BuiltInTopicType,
{
    dds_peek_read_take::<T, read_operation::Read>(reader)
}

pub fn dds_peek<T>(
    reader: cyclonedds_sys::dds_entity_t,
) -> Result<Vec<crate::sample::SampleOrKey<T>>>
where
    T: crate::builtin::private::BuiltInTopicType,
{
    dds_peek_read_take::<T, read_operation::Peek>(reader)
}

unsafe extern "C" fn builtin_read_with_collector_callback<T>(
    arg: *mut std::ffi::c_void,
    info: *const cyclonedds_sys::dds_sample_info_t,
    sertype: *const cyclonedds_sys::ddsi_sertype,
    serdata: *mut cyclonedds_sys::ddsi_serdata,
) -> cyclonedds_sys::dds_return_t
where
    T: crate::builtin::private::BuiltInTopicType,
{
    let samples = unsafe { &mut *(arg.cast::<Vec<crate::sample::SampleOrKey<T>>>()) };
    let info = unsafe { &*info };

    let valid_data = info.valid_data;
    let info: crate::sample::Info = info.into();

    let mut raw = T::Type::default();
    // NOTE: Cyclone zero initializes the non-key fields of the built-in samples in the keyed case.
    if unsafe {
        cyclonedds_sys::ddsi_serdata_to_sample(
            serdata,
            (&raw mut raw).cast(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
    } {
        let data = unsafe { T::from_ffi(raw) };
        if valid_data {
            samples.push(crate::sample::SampleOrKey::new_sample(data, info));
        } else {
            samples.push(crate::sample::SampleOrKey::new_key(data.as_key(), info));
        }

        unsafe {
            cyclonedds_sys::ddsi_sertype_free_sample(
                sertype,
                (&raw mut raw).cast(),
                cyclonedds_sys::DDS_FREE_CONTENTS_BIT,
            );
        }
        cyclonedds_sys::DDS_RETCODE_OK.cast_signed()
    } else {
        cyclonedds_sys::DDS_RETCODE_ERROR
    }
}
