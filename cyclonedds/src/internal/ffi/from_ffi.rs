use crate::QoS;
use crate::builtin::private::BuiltInTopicType;
use crate::builtin::{
    BuiltInTopicKey, DcpsEndpoint, DcpsParticipant, DcpsPublication, DcpsSubscription, DcpsTopic,
};

mod qos;

/// Converts raw FFI values into safe Rust types.
///
/// # Safety
///
/// Implementors must ensure `from_ffi` only reads valid FFI values and safely
/// acquires and releases resources in the conversion.
pub unsafe trait FromFfi {
    type Source;

    unsafe fn from_ffi(source: Self::Source) -> Self;
}

unsafe impl FromFfi for String {
    type Source = Option<std::ptr::NonNull<std::ffi::c_char>>;

    unsafe fn from_ffi(source: Self::Source) -> Self {
        source
            .map(|source| unsafe {
                std::ffi::CStr::from_ptr(source.as_ref())
                    .to_string_lossy()
                    .into_owned()
            })
            .unwrap_or_default()
    }
}

unsafe impl FromFfi for DcpsParticipant {
    type Source = <Self as BuiltInTopicType>::Type;

    unsafe fn from_ffi(source: Self::Source) -> Self {
        let key = BuiltInTopicKey::from_bytes(source.key.v);
        let qos = unsafe { QoS::from_ffi(std::ptr::NonNull::new(source.qos)) };
        Self { key, qos }
    }
}

unsafe impl FromFfi for DcpsTopic {
    type Source = <Self as BuiltInTopicType>::Type;

    unsafe fn from_ffi(source: Self::Source) -> Self {
        let key = BuiltInTopicKey::from_bytes(source.key.d);
        let topic_name = unsafe { String::from_ffi(std::ptr::NonNull::new(source.topic_name)) };
        let type_name = unsafe { String::from_ffi(std::ptr::NonNull::new(source.type_name)) };
        let qos = unsafe { QoS::from_ffi(std::ptr::NonNull::new(source.qos)) };
        Self {
            key,
            topic_name,
            type_name,
            qos,
        }
    }
}

unsafe impl FromFfi for DcpsPublication {
    type Source = <Self as BuiltInTopicType>::Type;

    unsafe fn from_ffi(source: Self::Source) -> Self {
        Self {
            endpoint: unsafe { DcpsEndpoint::from_ffi(source) },
        }
    }
}

unsafe impl FromFfi for DcpsSubscription {
    type Source = <Self as BuiltInTopicType>::Type;

    unsafe fn from_ffi(source: Self::Source) -> Self {
        Self {
            endpoint: unsafe { DcpsEndpoint::from_ffi(source) },
        }
    }
}

unsafe impl FromFfi for DcpsEndpoint {
    type Source = cyclonedds_sys::dds_builtintopic_endpoint_t;

    unsafe fn from_ffi(source: Self::Source) -> Self {
        let key = BuiltInTopicKey::from_bytes(source.key.v);
        let participant_key = BuiltInTopicKey::from_bytes(source.participant_key.v);
        let participant_instance_handle = crate::entity::InstanceHandle {
            inner: source.participant_instance_handle,
        };
        let topic_name = unsafe { String::from_ffi(std::ptr::NonNull::new(source.topic_name)) };
        let type_name = unsafe { String::from_ffi(std::ptr::NonNull::new(source.type_name)) };
        let qos = unsafe { QoS::from_ffi(std::ptr::NonNull::new(source.qos)) };
        Self {
            key,
            participant_key,
            participant_instance_handle,
            topic_name,
            type_name,
            qos,
        }
    }
}
