mod domain;
mod grid;
mod solve;
mod validate;

pub(crate) use crate::domain::Domain;

pub use crate::{
    grid::Grid,
    validate::{Cage, Operator},
};

const N: usize = 4;

pub fn main() {
    let grid = Grid([[0u8; N]; N]);

    let cages = vec![];

    let solved = grid.solve(&cages).unwrap();
    assert_eq!(validate::State::Valid, validate::all(&cages, &solved));
}
