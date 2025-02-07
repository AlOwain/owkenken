#![allow(unused_imports)]
use super::Domain;
use crate::{Cage, Grid, Operator};

#[test]
fn empty_grid() {
    const N: usize = 3;
    let grid = Grid([[0u8; N]; N]);
    let cages = vec![];

    let domain = Domain::new(&grid, &cages);

    for row in domain.iter() {
        for cell in row.iter() {
            assert_eq!(cell.len(), N);
            assert!(cell.iter().all(|&v| v));
        }
    }
}

#[test]
fn grid_constraints() {
    let grid = Grid([[1, 0, 0], [0, 2, 0], [0, 0, 3]]);
    let cages = vec![];

    let domain = Domain::new(&grid, &cages);

    assert_eq!(domain[0][0], [false, false, false]);
    assert_eq!(domain[0][1], [false, false, true]);
    assert_eq!(domain[0][2], [false, true, false]);

    assert_eq!(domain[1][0], [false, false, true]);
    assert_eq!(domain[1][1], [false, false, false]);
    assert_eq!(domain[1][2], [true, false, false]);

    assert_eq!(domain[2][0], [false, true, false]);
    assert_eq!(domain[2][1], [true, false, false]);
    assert_eq!(domain[2][2], [false, false, false]);
}

// #[test]
fn partially_filled_grid() {
    const N: usize = 4;
    let cages = vec![
        Cage {
            sign: Operator::Add,
            target: 6,
            coords: vec![(0, 0), (0, 1)],
        },
        Cage {
            sign: Operator::Multiply,
            target: 12,
            coords: vec![(1, 0), (1, 1)],
        },
    ];
    let mut grid = Grid([[0; N]; N]);
    grid[0][0] = 2;

    let domain = Domain::new(&grid, &cages);

    // FIXME: This does not work because the domain is not made with cages taken into consideration
    assert_eq!(domain[0][0], [false, false, false, false]);
    assert_eq!(domain[0][1], [false, false, false, true]);
    assert_eq!(domain[1][0], [false, false, false, false]);
}

#[test]
fn operator_none() {
    let cages = vec![
        Cage {
            sign: Operator::None,
            target: 2,
            coords: vec![(0, 0)],
        },
        Cage {
            sign: Operator::None,
            target: 1,
            coords: vec![(1, 1)],
        },
    ];
    let grid = Grid([[0, 0], [0, 0]]);

    let domain = Domain::new(&grid, &cages);

    assert_eq!(domain[0][0], [false, true]);
    assert_eq!(domain[0][1], [true, true]);
    assert_eq!(domain[1][0], [true, true]);
    assert_eq!(domain[1][1], [true, false]);
}

// TODO(optimization):
//   You should recompute the affected domains
// By removing the values that have no reflected values in the same cage
// At step #1 you could remove the values by the cages
// R1: 1001  1001  1111  1111
// Then at step #2 you could also remove values that are affected in the same row
// R1: 1001  1001  0110  0110

#[test]
fn operator_subtract() {
    const N: usize = 4;
    let cages = vec![Cage {
        sign: Operator::Subtract,
        target: 3,
        coords: vec![(0, 0), (0, 1)],
    }];
    let grid = Grid([[0; N]; N]);

    let mut domain = Domain([[[true; N]; N]; N]);
    domain = domain.cage(&grid, &cages[0]);
    assert_eq!([true, false, false, true], domain[0][0]);
    assert_eq!([true, false, false, true], domain[0][1]);

    let mut domain = Domain([[[true; N]; N]; N]);
    domain[0][0][0] = false;
    domain = domain.cage(&grid, &cages[0]);
    assert_eq!([false, false, false, true], domain[0][0]);
    assert_eq!([true, false, false, false], domain[0][1]);
}

// TODO: Seperate this into multiple tests
mod divide {
    use crate::{Cage, Domain, Grid, Operator};
    #[allow(dead_code)]
    const N: usize = 4;

    #[test]
    fn cage_only() {
        let cages = vec![Cage {
            sign: Operator::Divide,
            target: 2,
            coords: vec![(0, 0), (0, 1)],
        }];
        let grid = Grid([[0; N]; N]);

        let domain = Domain([[[true; N]; N]; N]);
        let domain = domain.cage(&grid, &cages[0]);
        assert_eq!([true, true, false, true], domain[0][0]);
        assert_eq!([true, true, false, true], domain[0][1]);
    }

    #[test]
    fn constrained_cage() {
        let cages = vec![Cage {
            sign: Operator::Divide,
            target: 2,
            coords: vec![(0, 0), (0, 1)],
        }];
        let mut domain = Domain([[[true; N]; N]; N]);
        let grid = Grid([[0; N]; N]);

        domain[0][0][0] = false;
        domain = domain.cage(&grid, &cages[0]);
        assert_eq!([false, true, false, true], domain[0][0]);
        assert_eq!([true, true, false, true], domain[0][1]);

        domain[0][0][3] = false;
        domain = domain.cage(&grid, &cages[0]);
        assert_eq!([false, true, false, false], domain[0][0]);
        assert_eq!([true, false, false, true], domain[0][1]);
    }

    #[test]
    fn grid_taken() {
        let cages = vec![Cage {
            sign: Operator::Divide,
            target: 2,
            coords: vec![(0, 0), (0, 1)],
        }];
        let domain = Domain([[[true; N]; N]; N]);
        let mut grid = Grid([[0; N]; N]);
        grid[0][1] = 2;

        let d1 = domain.clone().cage(&grid, &cages[0]);
        assert_eq!([true, true, false, true], d1[0][0]);

        let d2 = domain.clone().orthogonal(&grid);
        assert_eq!([true, false, true, true], d2[0][0]);
        assert_eq!([false, false, false, false], d2[0][1]);

        let d3 = domain.clone().domain(&grid, &cages);
        assert_eq!([true, false, false, true], d3[0][0]);
        assert_eq!([false, false, false, false], d3[0][1]);

        grid[0][1] = 1;
        let d4 = domain.clone().domain(&grid, &cages);
        assert_eq!([false, true, false, false], d4[0][0]);
        assert_eq!([false, false, false, false], d4[0][1]);
    }
}
