/// An absolute point in time.
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct Time {
    pub(crate) inner: cyclonedds_sys::dds_time_t,
}

impl Time {
    /// A time that never happens.
    pub const NEVER: Self = Time {
        inner: cyclonedds_sys::TIME_NEVER,
    };

    /// Convert a timestamp in nano-seconds to a [`Time`].
    pub fn from_nanos(nanos: i64) -> Self {
        Self { inner: nanos }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_create() {
        let nanos = 1000000000;
        let time = Time::from_nanos(nanos);
        assert_eq!(time.inner, nanos);
    }
}
