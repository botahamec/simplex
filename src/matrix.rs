use std::num::NonZeroUsize;

use raise::yeet;

struct Matrix {
	m: NonZeroUsize,
	n: NonZeroUsize,
	buffer: Vec<f64>,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum NewMatrixError {
	ZeroSize,
	InconsistentSize,
}

impl Matrix {
	pub fn new(columns: &[&[f64]]) -> Result<Self, NewMatrixError> {
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
		Ok(Matrix { m, n, buffer })
	}

	pub fn identity(size: NonZeroUsize) -> Self {
		let n = size.get();
		let mut buffer = Vec::with_capacity(n * n);
		for i in 0..n {
			for j in 0..n {
				if i == j {
					buffer.push(1.0);
				} else {
					buffer.push(0.0);
				}
			}
		}

		Matrix {
			m: size,
			n: size,
			buffer,
		}
	}
}
