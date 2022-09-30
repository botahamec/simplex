use core::num::NonZeroU16;

/// Uses Stein's algorithm to calculate the gcd of two numbers
const fn gcd(mut a: u16, mut b: u16) -> u16 {
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

const fn lcm(a: u16, b: u16) -> u16 {
	let gcd = gcd(a, b);
	a * b / gcd
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fraction32 {
	numerator: i16,
	denominator: NonZeroU16,
}

impl Fraction32 {
	pub const ZERO: Self = unsafe { Self::new_unchecked(0, 1) };
	pub const ONE: Self = unsafe { Self::new_unchecked(1, 1) };

	pub fn new(numerator: i16, denominator: NonZeroU16) -> Self {
		let this = Self {
			numerator,
			denominator,
		};

		// check for a denominator that's too large
		assert!(denominator.get() <= i16::MAX.unsigned_abs());

		// simplify the fraction
		this.reduce()
	}

	pub const unsafe fn new_unchecked(numerator: i16, denominator: u16) -> Self {
		Self {
			numerator,
			denominator: NonZeroU16::new_unchecked(denominator),
		}
	}

	pub const fn numerator(self) -> i16 {
		self.numerator
	}

	pub const fn denominator(self) -> NonZeroU16 {
		self.denominator
	}

	/// Simplify the fraction
	pub fn reduce(self) -> Self {
		if self.numerator == 0 {
			return Self::ZERO;
		}

		let gcd = gcd(self.numerator.unsigned_abs(), self.denominator.get());
		let numerator = self.numerator / i16::try_from(gcd).unwrap();
		let denominator = self.denominator.get() / gcd;

		Self::new(numerator, denominator.try_into().unwrap())
	}
}
