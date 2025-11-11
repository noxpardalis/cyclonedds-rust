#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum CdrSize {
    Bounded(usize),
    Unbounded,
}

trait Padding {
    fn with_padding(self, align: usize) -> Self;
}

impl Padding for usize {
    #[inline]
    fn with_padding(self, align: usize) -> usize {
        self + (align - (self % align)) % align
    }
}

impl Padding for CdrSize {
    #[inline]
    fn with_padding(self, align: usize) -> Self {
        match self {
            CdrSize::Bounded(size) => CdrSize::Bounded(size.with_padding(align)),
            CdrSize::Unbounded => CdrSize::Unbounded,
        }
    }
}

macro_rules! impl_cdr_size_op {
    ($trait:ident, $method:ident, $op:tt) => {
        // CdrSize <op> CdrSize
        impl $trait<CdrSize> for CdrSize {
            type Output = Self;

            fn $method(self, rhs: CdrSize) -> Self::Output {
                match (self, rhs) {
                    (CdrSize::Bounded(lhs), CdrSize::Bounded(rhs)) => CdrSize::Bounded(lhs $op rhs),
                    _ => CdrSize::Unbounded,
                }
            }
        }

        // CdrSize <op> usize
        impl $trait<usize> for CdrSize {
            type Output = Self;

            fn $method(self, rhs: usize) -> Self::Output {
                match self {
                    CdrSize::Bounded(lhs) => CdrSize::Bounded(lhs $op rhs),
                    _ => CdrSize::Unbounded,
                }
            }
        }

        // usize <op> CdrSize
        impl $trait<CdrSize> for usize {
            type Output = CdrSize;

            fn $method(self, rhs: CdrSize) -> Self::Output {
                match rhs {
                    CdrSize::Bounded(r) => CdrSize::Bounded(self $op r),
                    _ => CdrSize::Unbounded,
                }
            }
        }
    };
}

use std::ops::{Add, Div, Mul, Sub};
impl_cdr_size_op!(Add, add, +);
impl_cdr_size_op!(Sub, sub, -);
impl_cdr_size_op!(Mul, mul, *);
impl_cdr_size_op!(Div, div, /);

macro_rules! impl_cdr_size_op_assign {
    ($trait:ident, $method:ident, $op:tt) => {
        // CdrSize <op> CdrSize
        impl $trait<CdrSize> for CdrSize {
            fn $method(&mut self, rhs: CdrSize) {
                *self = *self $op rhs;
            }
        }
    }
}

use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};
impl_cdr_size_op_assign!(AddAssign, add_assign, +);
impl_cdr_size_op_assign!(DivAssign, div_assign, /);
impl_cdr_size_op_assign!(MulAssign, mul_assign, *);
impl_cdr_size_op_assign!(SubAssign, sub_assign, -);

///
pub trait CdrBounds {
    fn max_serialized_cdr_size() -> CdrSize;
    fn alignment() -> usize;
}

macro_rules! impl_cdr_bounds_by_size {
    ($t:ty, $size:expr) => {
        impl CdrBounds for $t {
            fn max_serialized_cdr_size() -> CdrSize {
                CdrSize::Bounded($size)
            }

            fn alignment() -> usize {
                $size
            }
        }
    };
}

impl_cdr_bounds_by_size!((), 0);
impl_cdr_bounds_by_size!(char, 1);
impl_cdr_bounds_by_size!(bool, 1);
impl_cdr_bounds_by_size!(u8, 1);
impl_cdr_bounds_by_size!(u16, 2);
impl_cdr_bounds_by_size!(u32, 4);
impl_cdr_bounds_by_size!(u64, 8);
impl_cdr_bounds_by_size!(i8, 1);
impl_cdr_bounds_by_size!(i16, 2);
impl_cdr_bounds_by_size!(i32, 4);
impl_cdr_bounds_by_size!(i64, 8);
impl_cdr_bounds_by_size!(f32, 4);
impl_cdr_bounds_by_size!(f64, 8);

impl<T, const N: usize> CdrBounds for [T; N]
where
    T: CdrBounds,
{
    fn max_serialized_cdr_size() -> CdrSize {
        T::max_serialized_cdr_size().with_padding(T::alignment()) * N
    }

    fn alignment() -> usize {
        T::alignment()
    }
}

macro_rules! impl_cdr_bounds_tuple {
    ($head:ident $(, $tail:ident)* $(,)?) => {
        impl<$head: CdrBounds $(, $tail: CdrBounds)*> CdrBounds for ($head, $($tail, )*) {
            fn max_serialized_cdr_size() -> CdrSize {
                $head::max_serialized_cdr_size().with_padding($head::alignment()) $(+ $tail::max_serialized_cdr_size().with_padding($tail::alignment()))*
            }

            fn alignment() -> usize {
                let alignment = 0;
                $(let alignment = alignment.max($tail::alignment());)*
                alignment
            }
        }

        impl_cdr_bounds_tuple!($($tail),*);
    };

    () => {};
}

impl_cdr_bounds_tuple! { T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, }

impl<T> CdrBounds for Vec<T>
where
    T: CdrBounds,
{
    fn max_serialized_cdr_size() -> CdrSize {
        CdrSize::Unbounded
    }

    fn alignment() -> usize {
        // max between alignment of length and type.
        u32::alignment().max(T::alignment())
    }
}

impl CdrBounds for String {
    fn max_serialized_cdr_size() -> CdrSize {
        CdrSize::Unbounded
    }

    fn alignment() -> usize {
        // length field as alignment
        u32::alignment()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cdr_size_bounds_comparisons() {
        let min_bounded = CdrSize::Bounded(usize::MIN);
        let max_bounded = CdrSize::Bounded(usize::MAX);
        let unbounded = CdrSize::Unbounded;

        assert!(min_bounded < max_bounded);
        assert!(min_bounded < unbounded);

        assert!(max_bounded > min_bounded);
        assert!(max_bounded < unbounded);

        assert!(unbounded > min_bounded);
        assert!(unbounded > max_bounded);
    }
}
