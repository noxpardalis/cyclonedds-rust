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
