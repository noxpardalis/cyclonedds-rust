/// A relative span of time.
///
/// This is typically used in DDS to represent timeouts.
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct Duration {
    pub(crate) inner: cyclonedds_sys::dds_duration_t,
}

impl Duration {
    /// A timeout of infinite duration.
    pub const INFINITE: Self = Duration {
        inner: cyclonedds_sys::DURATION_INFINITE,
    };

    /// Convert a timestamp in nano-seconds to a [`Duration`].
    ///
    /// NOTE: this function expects an [`i64`] for the nano-seconds which is
    /// distinct from [`std::time::Duration::from_nanos`] which expects a
    /// [`u64`].
    pub fn from_nanos(nanos: i64) -> Self {
        Self { inner: nanos }
    }
}

impl TryFrom<std::time::Duration> for Duration {
    type Error = crate::Error;

    fn try_from(value: std::time::Duration) -> Result<Self, Self::Error> {
        let inner = cyclonedds_sys::dds_duration_t::try_from(value.as_nanos())
            .map_err(|_| crate::Error::BadParameter)?;

        if inner == Self::INFINITE.inner {
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
    fn test_duration_create() {
        let nanos = 1000000000;
        let duration = Duration::from_nanos(nanos);
        assert_eq!(duration.inner, nanos);
    }

    #[test]
    fn test_duration_from_std_duration() {
        let nanos = 1000000000;
        let standard = std::time::Duration::from_nanos(nanos as u64);
        let duration = Duration::try_from(standard).unwrap();
        assert_eq!(duration.inner, nanos);
    }

    #[test]
    fn test_duration_from_out_of_range_std_duration() {
        let nanos = u64::MAX;
        let standard = std::time::Duration::from_nanos(nanos as u64);
        let result = Duration::try_from(standard).unwrap_err();
        assert_eq!(result, crate::Error::BadParameter);
    }
}
