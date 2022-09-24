mod matrix;

#[macro_export]
macro_rules! bail {
	($x: expr) => {
		return Err(($x).into())
	};
}
