use core::fmt::{self, Display};
use core::num::ParseIntError;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use core::str::FromStr;
use std::num::NonZeroUsize;

/// Uses Stein's algorithm to calculate the gcd of two numbers
const fn gcd(mut a: usize, mut b: usize) -> usize {
	// returns the other if one of the two numbers are zero
	if a == 0 || b == 0 {
		return a | b;
	}

	// find common factors of two
	let shift = (a | b).trailing_zeros();

	// divide both by two until they're odd
	a >>= a.trailing_zeros();
	b >>= b.trailing_zeros();

	while a != b {
		if a > b {
			a -= b;
			a >>= a.trailing_zeros();
		} else {
			b -= a;
			b >>= b.trailing_zeros();
		}
	}

	a << shift
}

const fn lcm(a: usize, b: usize) -> usize {
	let gcd = gcd(a, b);
	a * b / gcd
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fraction {
	numerator: isize,
	denominator: NonZeroUsize,
}

impl Fraction {
	pub const ZERO: Self = Self::whole(0);
	pub const ONE: Self = Self::whole(1);
	pub const NEG_ONE: Self = Self::whole(-1);

	/// Create a new fraction
	///
	/// # Panics
	///
	/// This panics if the denominator is larger than `isize::MAX`
	#[must_use]
	pub fn new(numerator: isize, denominator: NonZeroUsize) -> Self {
		let this = Self {
			numerator,
			denominator,
		};

		// check for a denominator that's too large
		assert!(denominator.get() <= isize::MAX.unsigned_abs());

		// simplify the fraction
		this.reduce()
	}

	/// Create a fraction from a whole number
	#[must_use]
	pub const fn whole(num: isize) -> Self {
		// safety: one is neither zero, nor greater than 35,000
		unsafe { Self::new_unchecked(num, 1) }
	}

	/// Create a new fraction
	///
	/// # Safety
	///
	/// The `denominator` cannot be zero, or larger than `i16::MAX`
	#[must_use]
	pub const unsafe fn new_unchecked(numerator: isize, denominator: usize) -> Self {
		Self {
			numerator,
			denominator: NonZeroUsize::new_unchecked(denominator),
		}
	}

	#[must_use]
	pub const fn numerator(self) -> isize {
		self.numerator
	}

	#[must_use]
	pub const fn denominator(self) -> NonZeroUsize {
		self.denominator
	}

	/// Simplify the fraction
	#[must_use]
	#[allow(clippy::missing_panics_doc)]
	pub fn reduce(self) -> Self {
		if self.numerator == 0 {
			return Self::ZERO;
		}

		let gcd = gcd(self.numerator.unsigned_abs(), self.denominator.get());
		let numerator = self.numerator / isize::try_from(gcd).unwrap();
		let denominator = self.denominator.get() / gcd;

		Self::new(numerator, denominator.try_into().unwrap())
	}

	/// Returns the reciprocal of the fraction.
	/// Returns `None` if the numerator is currently zero.
	#[must_use]
	#[allow(clippy::missing_panics_doc)]
	pub fn reciprocal(self) -> Option<Self> {
		let numerator = isize::try_from(self.denominator.get()).unwrap() * self.numerator.signum();
		let denominator = self.numerator.unsigned_abs().try_into().ok()?;

		Some(Self::new(numerator, denominator))
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseFractionError {
	BadInteger(ParseIntError),
	ZeroDenominator,
}

impl From<ParseIntError> for ParseFractionError {
	fn from(e: ParseIntError) -> Self {
		Self::BadInteger(e)
	}
}

impl FromStr for Fraction {
	type Err = ParseFractionError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if let Some((numerator, denominator)) = s.split_once('/') {
			let numerator = numerator.parse()?;
			let denominator = denominator.parse()?;
			let denominator =
				NonZeroUsize::new(denominator).ok_or(ParseFractionError::ZeroDenominator)?;

			Ok(Self::new(numerator, denominator))
		} else {
			Ok(Self::whole(s.parse()?))
		}
	}
}

impl PartialOrd<Self> for Fraction {
	fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
		let lcm = lcm(self.denominator.get(), other.denominator.get());
		let self_scale: isize = (lcm / self.denominator).try_into().ok()?;
		let other_scale: isize = (lcm / other.denominator).try_into().ok()?;

		(self.numerator * self_scale).partial_cmp(&(other.numerator * other_scale))
	}
}

impl Ord for Fraction {
	fn cmp(&self, other: &Self) -> core::cmp::Ordering {
		self.partial_cmp(other).unwrap()
	}
}

impl Display for Fraction {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}/{}", self.numerator, self.denominator)
	}
}

impl From<isize> for Fraction {
	fn from(v: isize) -> Self {
		Self::whole(v)
	}
}

impl Add<Self> for Fraction {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		let denominator = lcm(self.denominator.get(), rhs.denominator.get());
		let self_scale: isize = (denominator / self.denominator).try_into().ok().unwrap();
		let other_scale: isize = (denominator / rhs.denominator).try_into().ok().unwrap();
		let numerator = self.numerator * self_scale + rhs.numerator * other_scale;
		Self::new(numerator, NonZeroUsize::new(denominator).unwrap())
	}
}

impl AddAssign<Self> for Fraction {
	fn add_assign(&mut self, rhs: Self) {
		*self = *self + rhs;
	}
}

impl Sub<Self> for Fraction {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		self.add(rhs.mul(Self::NEG_ONE))
	}
}

impl SubAssign<Self> for Fraction {
	fn sub_assign(&mut self, rhs: Self) {
		*self = *self - rhs;
	}
}

impl Mul<Self> for Fraction {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		let numerator = self.numerator * rhs.numerator;
		let denominator = self.denominator.checked_mul(rhs.denominator).unwrap();

		Self::new(numerator, denominator)
	}
}

impl MulAssign<Self> for Fraction {
	fn mul_assign(&mut self, rhs: Self) {
		*self = *self * rhs;
	}
}

impl Div<Self> for Fraction {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		self.mul(rhs.reciprocal().unwrap())
	}
}

impl DivAssign<Self> for Fraction {
	fn div_assign(&mut self, rhs: Self) {
		*self = *self / rhs;
	}
}
