use super::*;

#[test]
fn test_ddsi_sertype_new_ref_unref_fini() {
    let mut sertype = ddsi_sertype_new(
        c"data",
        &crate::internal::sertype::Sertype::<crate::tests::topic::Data>::SERTYPE_OPS,
        &crate::internal::sertype::Sertype::<crate::tests::topic::Data>::SERDATA_OPS,
        false,
    );
    assert_eq!(sertype.flags_refc.v, 1);
    ddsi_sertype_ref(&mut sertype);
    assert_eq!(sertype.flags_refc.v, 2);
    ddsi_sertype_unref(&mut sertype);
    assert_eq!(sertype.flags_refc.v, 1);
    ddsi_sertype_fini(&mut sertype);
}

#[test]
fn test_ddsi_serdata_new_ref_unref() {
    let mut sertype = ddsi_sertype_new(
        c"data",
        &crate::internal::sertype::Sertype::<crate::tests::topic::Data>::SERTYPE_OPS,
        &crate::internal::sertype::Sertype::<crate::tests::topic::Data>::SERDATA_OPS,
        false,
    );

    let mut serdata = ddsi_serdata_new(&sertype, crate::internal::serdata::Kind::Empty.into());
    assert_eq!(serdata.refc.v, 1);
    ddsi_serdata_ref(&mut serdata);
    assert_eq!(serdata.refc.v, 2);
    ddsi_serdata_unref(&mut serdata);
    assert_eq!(serdata.refc.v, 1);
    ddsi_sertype_fini(&mut sertype);
}

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

    let mut vec: Vec<
        Result<crate::sample::Sample<crate::tests::topic::Data>, crate::sample::Info>,
    > = Vec::new();
    let arg = &mut vec as *mut Vec<_> as *mut std::ffi::c_void;
    let mut info = cyclonedds_sys::dds_sample_info_t::default();
    info.valid_data = true;

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
fn test_dds_read_with_collector_on_invalid_sample() {
    let sertype =
        crate::internal::sertype::Sertype::<crate::tests::topic::Data>::new(c"Data", true);
    let mut serdata =
        crate::internal::serdata::Serdata::new(&sertype, crate::internal::serdata::Kind::Empty);

    let mut vec: Vec<
        Result<crate::sample::Sample<crate::tests::topic::Data>, crate::sample::Info>,
    > = Vec::new();
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
    .unwrap();

    assert_eq!(result, cyclonedds_sys::DDS_RETCODE_OK as _);
    assert_eq!(vec.len(), 1);
    assert_eq!(
        vec[0],
        Err((&cyclonedds_sys::dds_sample_info_t::default()).into())
    );

    let sample: crate::tests::topic::Data = Default::default();
    serdata.sample = Some(std::sync::Arc::new(sample.clone()));
    info.valid_data = true;
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

    assert_eq!(result, cyclonedds_sys::DDS_RETCODE_OK as _);
    assert_eq!(vec.len(), 2);
    assert_eq!(
        vec[0],
        Err((&cyclonedds_sys::dds_sample_info_t::default()).into())
    );
    assert_eq!(*vec[1].clone().unwrap(), Default::default());
}

#[test]
fn test_dds_read_take_peek_on_invalid_entity() {
    let result = dds_read_take_peek::<crate::tests::topic::Data, read_operation::Peek>(0);
    assert!(result.is_err());
}
