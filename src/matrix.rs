struct Matrix {
	m: usize,
	n: usize,
	buffer: Vec<f64>,
}

impl Matrix {
	pub fn identity(n: usize) -> Matrix {
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

		Matrix { m: n, n, buffer }
	}
}
