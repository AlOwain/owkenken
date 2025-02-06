mod cage;
mod tests;

use crate::Grid;
pub use cage::{Cage, Operator};

#[derive(Debug, PartialEq)]
pub enum State {
    Satisfiable,
    Unsatisfiable,
    Valid,
}
use State::*;

fn orthogonal<const N: usize>(grid: &Grid<N>) -> State {
    let mut valid = true;
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            if grid[i][j] == 0 {
                valid = false;
                continue;
            }
            for k in j + 1..grid.len() {
                if grid[i][j] == grid[i][k] {
                    return Unsatisfiable;
                }
                if grid[j][i] == grid[k][i] {
                    return Unsatisfiable;
                }
            }
        }
    }

    if valid {
        Valid
    } else {
        Satisfiable
    }
}

fn cages<const N: usize>(cages: &[Cage], grid: &Grid<N>) -> State {
    let mut valid = true;
    for cage in cages {
        match cage.validate(grid) {
            Valid => (),
            Satisfiable => valid = false,
            Unsatisfiable => return Unsatisfiable,
        }
    }

    if valid {
        Valid
    } else {
        Satisfiable
    }
}

fn domain<const N: usize>(grid: &Grid<N>) -> State {
    let mut valid = true;
    for row in grid.iter() {
        for item in row {
            if *item > grid.len() as u8 {
                return Unsatisfiable;
            }
            if *item <= 0 {
                valid = false;
            }
        }
    }

    if valid {
        Valid
    } else {
        Satisfiable
    }
}

pub fn all<const N: usize>(cage: &[Cage], grid: &Grid<N>) -> State {
    let mut valid = true;
    let orthogonal_res = orthogonal(grid);
    if orthogonal_res == Unsatisfiable {
        return Unsatisfiable;
    } else if orthogonal_res == Satisfiable {
        valid = false;
    }

    let cages_res = cages(cage, grid);
    if cages_res == Unsatisfiable {
        return Unsatisfiable;
    } else if cages_res == Satisfiable {
        valid = false;
    }

    let domain_res = domain(grid);
    if domain_res == Unsatisfiable {
        return Unsatisfiable;
    } else if domain_res == Satisfiable {
        valid = false;
    }

    if valid {
        Valid
    } else {
        Satisfiable
    }
}
