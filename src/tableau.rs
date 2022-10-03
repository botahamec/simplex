use crate::fraction::Fraction32;
use crate::matrix::Matrix;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Tableau {
	left: Matrix,
	right: Matrix,
	bottom_row: Vec<Fraction32>,
	right_column: Vec<Fraction32>,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum NewTableauError {}

impl Tableau {
	pub fn new() {}

	pub fn update_tableau() {}

	pub fn find_pivot() {}
}
