#[allow(non_snake_case)]
#[cfg(test)]
use crate::{
    validate::{self, Cage, Operator, State::*},
    Grid,
};

#[test]
fn equal_4x4() {
    let grid = Grid([[1u8; 4]; 4]);
    assert_eq!(Unsatisfiable, validate::all(&[], &grid));
}

#[test]
fn all_2x2() {
    const N: usize = 2;
    let grids: Vec<Grid<N>> = vec![
        Grid([[1, 1], [1, 1]]),
        Grid([[2, 1], [1, 1]]),
        Grid([[1, 2], [1, 1]]),
        Grid([[2, 2], [1, 1]]),
        Grid([[1, 1], [2, 1]]),
        Grid([[2, 1], [2, 1]]),
        Grid([[1, 2], [2, 1]]),
        Grid([[2, 2], [2, 1]]),
    ];

    for grid in grids {
        if grid == Grid([[1, 2], [2, 1]]) {
            assert_eq!(Valid, validate::all(&[], &grid));
        } else {
            assert_eq!(Unsatisfiable, validate::all(&[], &grid));
        }
    }
}

#[test]
fn subtract() {
    const N: usize = 4;
    let cages = vec![Cage {
        sign: Operator::Subtract,
        target: 3,
        coords: vec![(0, 0), (0, 1)],
    }];
    let mut grid = Grid([[0; N]; N]);
    grid[0][0] = 4;
    grid[0][1] = 1;
    assert_eq!(Valid, validate::cages(&cages, &grid));
    grid[0][0] = 1;
    grid[0][1] = 4;
    assert_eq!(Valid, validate::cages(&cages, &grid));
}

#[test]
fn latin_square() {
    const N: usize = 4;
    let mut grid = Grid([[0u8; N]; N]);
    let mut shift: usize = 0;

    for row in grid.iter_mut() {
        for i in 0..row.len() {
            row[(i + shift) % 4] = (i as u8) + 1;
        }
        shift += 1;
    }

    assert_eq!(Valid, validate::all(&vec![], &grid));
}

#[test]
fn sum_3x3() {
    use validate::{Cage, Operator};
    let grid = Grid([[1, 3, 2], [2, 1, 3], [3, 2, 1]]);

    let cages = vec![
        Cage {
            sign: Operator::Add,
            target: 3,
            coords: vec![(0, 0), (1, 0)],
        },
        Cage {
            sign: Operator::Add,
            target: 5,
            coords: vec![(0, 1), (0, 2)],
        },
        Cage {
            sign: Operator::Add,
            target: 4,
            coords: vec![(1, 1), (1, 2)],
        },
        Cage {
            sign: Operator::Add,
            target: 5,
            coords: vec![(2, 0), (2, 1)],
        },
        Cage {
            sign: Operator::None,
            target: 1,
            coords: vec![(2, 2)],
        },
    ];

    assert_eq!(Valid, validate::all(&cages, &grid));
}

#[test]
fn mixed_3x3() {
    use validate::{Cage, Operator};
    let grid = Grid([[3, 1, 2], [1, 2, 3], [2, 3, 1]]);

    let cages = vec![
        Cage {
            sign: Operator::Multiply,
            target: 3,
            coords: vec![(0, 0), (0, 1), (1, 0)],
        },
        Cage {
            sign: Operator::Add,
            target: 7,
            coords: vec![(0, 2), (1, 1), (1, 2)],
        },
        Cage {
            sign: Operator::None,
            target: 2,
            coords: vec![(2, 0)],
        },
        Cage {
            sign: Operator::Divide,
            target: 3,
            coords: vec![(2, 1), (2, 2)],
        },
    ];

    assert_eq!(Valid, validate::all(&cages, &grid));
}

#[test]
fn mixed_9x9() {
    use validate::{Cage, Operator};
    let grid = Grid([
        [9, 3, 8, 2, 4, 1, 7, 6, 5],
        [1, 2, 3, 4, 5, 9, 6, 7, 8],
        [2, 1, 9, 7, 8, 4, 5, 3, 6],
        [6, 8, 4, 3, 2, 7, 9, 5, 1],
        [4, 9, 2, 6, 3, 5, 1, 8, 7],
        [8, 4, 1, 5, 7, 6, 3, 9, 2],
        [5, 7, 6, 9, 1, 3, 8, 2, 4],
        [7, 6, 5, 1, 9, 8, 2, 4, 3],
        [3, 5, 7, 8, 6, 2, 4, 1, 9],
    ]);

    let cages = vec![
        Cage {
            sign: Operator::Divide,
            target: 3,
            coords: vec![(0, 0), (0, 1)],
        },
        Cage {
            sign: Operator::Subtract,
            target: 5,
            coords: vec![(0, 2), (1, 2)],
        },
        Cage {
            sign: Operator::Divide,
            target: 2,
            coords: vec![(0, 3), (0, 4)],
        },
        Cage {
            sign: Operator::Subtract,
            target: 8,
            coords: vec![(0, 5), (1, 5)],
        },
        Cage {
            sign: Operator::Multiply,
            target: 210,
            coords: vec![(0, 6), (0, 7), (0, 8)],
        },
        Cage {
            sign: Operator::Add,
            target: 3,
            coords: vec![(1, 0), (1, 1)],
        },
        Cage {
            sign: Operator::Multiply,
            target: 160,
            coords: vec![(1, 3), (1, 4), (2, 4)],
        },
        Cage {
            sign: Operator::Multiply,
            target: 120,
            coords: vec![(2, 5), (2, 6), (1, 6)],
        },
        Cage {
            sign: Operator::Subtract,
            target: 1,
            coords: vec![(1, 7), (1, 8)],
        },
        Cage {
            sign: Operator::Divide,
            target: 3,
            coords: vec![(2, 0), (3, 0)],
        },
        Cage {
            sign: Operator::None,
            target: 1,
            coords: vec![(2, 1)],
        },
        Cage {
            sign: Operator::Multiply,
            target: 252,
            coords: vec![(3, 2), (2, 2), (2, 3)],
        },
        Cage {
            sign: Operator::Multiply,
            target: 135,
            coords: vec![(3, 6), (3, 7), (2, 7)],
        },
        Cage {
            sign: Operator::Multiply,
            target: 6,
            coords: vec![(2, 8), (3, 8)],
        },
        Cage {
            sign: Operator::Add,
            target: 17,
            coords: vec![(3, 1), (4, 1)],
        },
        Cage {
            sign: Operator::Subtract,
            target: 3,
            coords: vec![(3, 3), (4, 3)],
        },
        Cage {
            sign: Operator::Subtract,
            target: 5,
            coords: vec![(3, 4), (3, 5)],
        },
        Cage {
            sign: Operator::Multiply,
            target: 128,
            coords: vec![(4, 0), (5, 0), (5, 1)],
        },
        Cage {
            sign: Operator::Add,
            target: 3,
            coords: vec![(4, 2), (5, 2)],
        },
        Cage {
            sign: Operator::Multiply,
            target: 15,
            coords: vec![(4, 4), (4, 5), (4, 6)],
        },
        Cage {
            sign: Operator::Add,
            target: 17,
            coords: vec![(4, 7), (5, 7)],
        },
        Cage {
            sign: Operator::Multiply,
            target: 56,
            coords: vec![(4, 8), (5, 8), (6, 8)],
        },
        Cage {
            sign: Operator::Multiply,
            target: 270,
            coords: vec![(5, 3), (6, 3), (6, 2)],
        },
        Cage {
            sign: Operator::None,
            target: 7,
            coords: vec![(5, 4)],
        },
        Cage {
            sign: Operator::Multiply,
            target: 144,
            coords: vec![(5, 5), (5, 6), (6, 6)],
        },
        Cage {
            sign: Operator::Multiply,
            target: 245,
            coords: vec![(6, 0), (6, 1), (7, 0)],
        },
        Cage {
            sign: Operator::Divide,
            target: 3,
            coords: vec![(6, 4), (6, 5)],
        },
        Cage {
            sign: Operator::None,
            target: 2,
            coords: vec![(6, 7)],
        },
        Cage {
            sign: Operator::Add,
            target: 12,
            coords: vec![(7, 1), (7, 2), (7, 3)],
        },
        Cage {
            sign: Operator::Add,
            target: 17,
            coords: vec![(7, 4), (7, 5)],
        },
        Cage {
            sign: Operator::Multiply,
            target: 24,
            coords: vec![(7, 6), (7, 7), (7, 8)],
        },
        Cage {
            sign: Operator::Subtract,
            target: 2,
            coords: vec![(8, 0), (8, 1)],
        },
        Cage {
            sign: Operator::Add,
            target: 21,
            coords: vec![(8, 2), (8, 3), (8, 4)],
        },
        Cage {
            sign: Operator::Subtract,
            target: 2,
            coords: vec![(8, 5), (8, 6)],
        },
        Cage {
            sign: Operator::Subtract,
            target: 8,
            coords: vec![(8, 7), (8, 8)],
        },
    ];

    assert_eq!(validate::all(&cages, &grid), Valid);
}
