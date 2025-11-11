#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum CdrSize {
    Bounded(usize),
    Unbounded,
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

///
pub trait CdrBounds {
    fn max_serialized_cdr_size() -> CdrSize;
}

macro_rules! impl_cdr_bounds_by_size {
    ($t:ty) => {
        impl CdrBounds for $t {
            fn max_serialized_cdr_size() -> CdrSize {
                CdrSize::Bounded(std::mem::size_of::<Self>())
            }
        }
    };
}

impl_cdr_bounds_by_size!(());
impl_cdr_bounds_by_size!(bool);
impl_cdr_bounds_by_size!(u8);
impl_cdr_bounds_by_size!(u16);
impl_cdr_bounds_by_size!(u32);
impl_cdr_bounds_by_size!(u64);
impl_cdr_bounds_by_size!(i8);
impl_cdr_bounds_by_size!(i16);
impl_cdr_bounds_by_size!(i32);
impl_cdr_bounds_by_size!(i64);
impl_cdr_bounds_by_size!(f32);
impl_cdr_bounds_by_size!(f64);

impl<T, const N: usize> CdrBounds for [T; N]
where
    T: CdrBounds,
{
    fn max_serialized_cdr_size() -> CdrSize {
        T::max_serialized_cdr_size() * N + std::mem::size_of::<usize>()
    }
}

macro_rules! impl_cdr_bounds_tuple {
    ($head:ident $(, $tail:ident)* $(,)?) => {
        impl<$head: CdrBounds $(, $tail: CdrBounds)*> CdrBounds for ($head, $($tail, )*) {
            fn max_serialized_cdr_size() -> CdrSize {
                $head::max_serialized_cdr_size() $(+ $tail::max_serialized_cdr_size())*
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
}

impl CdrBounds for String {
    fn max_serialized_cdr_size() -> CdrSize {
        CdrSize::Unbounded
    }
}
