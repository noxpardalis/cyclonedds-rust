use super::*;

#[test]
fn test_dds_delete_on_non_existent_entity() {
    let entity = 101;
    let result = dds_delete(entity);
    assert!(result.is_err());
}

#[test]
fn test_dds_read_with_collector_on_empty_sample() {
    let sertype =
        crate::internal::sertype::Sertype::<crate::tests::topic::Data>::new(c"Data", true);
    let mut serdata =
        crate::internal::serdata::Serdata::new(&sertype, crate::internal::serdata::Kind::Empty);

    let mut vec: Vec<crate::sample::Sample<crate::tests::topic::Data>> = Vec::new();
    let arg = &mut vec as *mut Vec<_> as *mut std::ffi::c_void;
    let mut info = cyclonedds_sys::dds_sample_info_t::default();

    let result = unsafe {
        dds_read_with_collector_callback::<crate::tests::topic::Data>(
            arg,
            &mut info,
            &sertype.inner,
            &mut serdata.inner,
        )
    }
    .into_error()
    .unwrap_err();
    assert_eq!(result, crate::Error::PreconditionNotMet);

    serdata.sample = Some(Default::default());
    unsafe {
        dds_read_with_collector_callback::<crate::tests::topic::Data>(
            arg,
            &mut info,
            &sertype.inner,
            &mut serdata.inner,
        )
    }
    .into_error()
    .unwrap();
}

#[test]
fn test_dds_read_take_peek_on_invalid_entity() {
    let result = dds_read_take_peek::<crate::tests::topic::Data>(0, ReadOperation::Peek);
    assert!(result.is_err());
}
