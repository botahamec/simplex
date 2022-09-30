use core::num::NonZeroUsize;

use raise::yeet;

use crate::fraction::Fraction32;

struct Matrix {
	m: NonZeroUsize,
	n: NonZeroUsize,
	buffer: Vec<Fraction32>,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum NewMatrixError {
	ZeroSize,
	InconsistentSize,
}

impl Matrix {
	pub fn new(columns: &[&[Fraction32]]) -> Result<Self, NewMatrixError> {
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

	pub fn identity(size: NonZeroUsize) -> Self {
		let n = size.get();
		let mut buffer = Vec::with_capacity(n * n);
		for i in 0..n {
			for j in 0..n {
				if i == j {
					buffer.push(Fraction32::ONE);
				} else {
					buffer.push(Fraction32::ZERO);
				}
			}
		}

		Self {
			m: size,
			n: size,
			buffer,
		}
	}
}
