use crate::fraction::Fraction;
use crate::matrix::Matrix;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Tableau {
	left: Matrix,
	right: Matrix,
	bottom_row: Vec<Fraction>,
	right_column: Vec<Fraction>,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum NewTableauError {}

impl Tableau {
	pub fn new() {}

	pub fn update_tableau() {}

	pub fn find_pivot() {}
}
