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

impl TryFrom<std::time::SystemTime> for Time {
    type Error = crate::Error;

    fn try_from(value: std::time::SystemTime) -> Result<Self, Self::Error> {
        let value = value
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .map_err(|_| crate::Error::BadParameter)?;

        let inner = cyclonedds_sys::dds_time_t::try_from(value.as_nanos())
            .map_err(|_| crate::Error::BadParameter)?;

        if inner == Self::NEVER.inner {
            Err(crate::Error::BadParameter)
        } else {
            Ok(Self { inner })
        }
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
