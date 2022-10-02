use crate::fraction::Fraction32;
use crate::matrix::Matrix;

struct Tableau {
    left: Matrix,
    right: Matrix,
    botRow: Vec<Fraction32>,
    rightCol: Vec<Fraction32>,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum NewTableauError {

}


impl Tableau {
    pub fn new() {
        
    }

    pub fn updateTableau() {

    }

    pub fn findPivot() {

    }

    pub fn printTableau() {

    }
}