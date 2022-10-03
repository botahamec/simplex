use core::num::NonZeroUsize;

use raise::yeet;

use crate::fraction::Fraction;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Matrix {
	m: NonZeroUsize,
	n: NonZeroUsize,
	buffer: Vec<Fraction>,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum NewMatrixError {
	ZeroSize,
	InconsistentSize,
}

impl Matrix {
	/// Creates a new matrix
	///
	/// # Errors
	///
	/// Returns an error if the columns are inconsistently sized, or there are no columns, or no rows
	pub fn new(columns: &[&[Fraction]]) -> Result<Self, NewMatrixError> {
		let n = columns.len();
		let m = columns.get(0).ok_or(NewMatrixError::ZeroSize)?.len();
		let mut buffer = Vec::with_capacity(m * n);

		for column in columns {
			if column.len() != m {
				yeet!(NewMatrixError::InconsistentSize);
			}

			for j in 0..m {
				buffer.push(*column.get(j).ok_or(NewMatrixError::InconsistentSize)?);
			}
		}

		let m = NonZeroUsize::new(m).ok_or(NewMatrixError::ZeroSize)?;
		let n = NonZeroUsize::new(n).ok_or(NewMatrixError::ZeroSize)?;
		Ok(Self { m, n, buffer })
	}

	#[must_use]
	pub fn identity(size: NonZeroUsize) -> Self {
		let n = size.get();
		let mut buffer = Vec::with_capacity(n * n);
		for i in 0..n {
			for j in 0..n {
				if i == j {
					buffer.push(Fraction::ONE);
				} else {
					buffer.push(Fraction::ZERO);
				}
			}
		}

		Self {
			m: size,
			n: size,
			buffer,
		}
	}

	#[must_use]
	pub fn get(&self, row: usize, col: usize) -> Option<&Fraction> {
		if row < self.m.get() || col < self.n.get() {
			return None;
		}

		self.buffer.get(col * self.n.get() + row)
	}

	#[must_use]
	pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut Fraction> {
		if row < self.m.get() || col < self.n.get() {
			return None;
		}

		self.buffer.get_mut(col * self.n.get() + row)
	}

	#[must_use]
	#[allow(clippy::missing_panics_doc)]
	pub fn get_min(&self) -> &Fraction {
		return self.buffer.iter().min().unwrap();
	}

	#[allow(clippy::missing_panics_doc)]
	pub fn add_to_all(&mut self, value: &Fraction) {
		let n = self.n.get();
		let m = self.m.get();
		for i in 0..m {
			for j in 0..n {
				let t = self.get_mut(i, j).unwrap();
				*t += *value;
			}
		}
	}
}
