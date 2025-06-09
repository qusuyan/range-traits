use core::num::{
	NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
	NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

pub trait MaybeBounded: Sized {
	fn min() -> Option<Self>;
	fn max() -> Option<Self>;
}

pub trait Bounded {
	fn min() -> Self;
	fn max() -> Self;
}

impl<T: Bounded> MaybeBounded for T {
	fn min() -> Option<Self> {
		Some(Self::min())
	}

	fn max() -> Option<Self> {
		Some(Self::max())
	}
}

impl Bounded for char {
	fn min() -> Self {
		'\0'
	}

	fn max() -> Self {
		char::MAX
	}
}

macro_rules! impl_int {
	($($ty:ident),*) => {
		$(
			impl Bounded for $ty {
				fn min() -> Self {
					$ty::MIN
				}

				fn max() -> Self {
					$ty::MAX
				}
			}
		)*
	};
}

impl_int!(
	u8,
	u16,
	u32,
	u64,
	u128,
	usize,
	i8,
	i16,
	i32,
	i64,
	i128,
	isize,
	NonZeroU8,
	NonZeroU16,
	NonZeroU32,
	NonZeroU64,
	NonZeroU128,
	NonZeroUsize,
	NonZeroI8,
	NonZeroI16,
	NonZeroI32,
	NonZeroI64,
	NonZeroI128,
	NonZeroIsize
);

macro_rules! impl_float {
	($($ty:ident),*) => {
		$(
			impl Bounded for $ty {
				fn min() -> Self {
					$ty::NEG_INFINITY
				}

				fn max() -> Self {
					$ty::INFINITY
				}
			}
		)*
	};
}

impl_float!(f32, f64);
