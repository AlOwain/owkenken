#![allow(unused_imports)]

mod solve {
    use crate::{
        solve,
        validate::{self, Cage, Operator, State::*},
        Grid,
    };

    #[test]
    fn empty_3x3() {
        const N: usize = 3;
        let grid = Grid([[0u8; N]; N]);
        let cages = vec![];

        let solved = grid.solve(&cages).unwrap();
        assert_eq!(validate::State::Valid, validate::all(&cages, &solved));
    }

    #[test]
    fn t1() {
        const N: usize = 3;
        let grid = Grid([[0u8; N]; N]);
        let cages = vec![
            Cage {
                sign: Operator::Add,
                target: 3,
                coords: vec![(0, 0), (1, 0)],
            },
            Cage {
                sign: Operator::Add,
                target: 5,
                coords: vec![(0, 1), (1, 1)],
            },
            Cage {
                sign: Operator::None,
                target: 1,
                coords: vec![(0, 2)],
            },
            Cage {
                sign: Operator::Add,
                target: 5,
                coords: vec![(1, 2), (2, 2)],
            },
            Cage {
                sign: Operator::Add,
                target: 4,
                coords: vec![(2, 0), (2, 1)],
            },
        ];

        assert_eq!(validate::all(&cages, &grid.solve(&cages).unwrap()), Valid);
    }

    #[test]
    fn t2() {
        const N: usize = 4;
        let grid = Grid([[0u8; N]; N]);

        let cages = vec![
            Cage {
                sign: Operator::Multiply,
                target: 24,
                coords: vec![(0, 0), (0, 1), (1, 0)],
            },
            Cage {
                sign: Operator::Divide,
                target: 2,
                coords: vec![(0, 2), (0, 3)],
            },
            Cage {
                sign: Operator::Subtract,
                target: 3,
                coords: vec![(1, 1), (1, 2)],
            },
            Cage {
                sign: Operator::Subtract,
                target: 1,
                coords: vec![(1, 3), (2, 3)],
            },
            Cage {
                sign: Operator::Add,
                target: 5,
                coords: vec![(2, 0), (2, 1)],
            },
            Cage {
                sign: Operator::Add,
                target: 6,
                coords: vec![(2, 2), (3, 2), (3, 3)],
            },
            Cage {
                sign: Operator::Subtract,
                target: 3,
                coords: vec![(3, 0), (3, 1)],
            },
        ];

        let solved = grid.solve(&cages).unwrap();
        assert_eq!(Valid, validate::all(&cages, &solved));
    }

    #[test]
    fn t3() {
        const N: usize = 6;
        let grid = Grid([[0u8; N]; N]);

        let cages = vec![
            Cage {
                sign: Operator::Subtract,
                target: 4,
                coords: vec![(0, 0), (1, 0)],
            },
            Cage {
                sign: Operator::Subtract,
                target: 1,
                coords: vec![(0, 1), (1, 1)],
            },
            Cage {
                sign: Operator::Subtract,
                target: 3,
                coords: vec![(0, 2), (0, 3)],
            },
            Cage {
                sign: Operator::Divide,
                target: 3,
                coords: vec![(0, 4), (0, 5)],
            },
            Cage {
                sign: Operator::Subtract,
                target: 1,
                coords: vec![(1, 2), (1, 3)],
            },
            Cage {
                sign: Operator::Multiply,
                target: 150,
                coords: vec![(1, 4), (2, 4), (1, 5)],
            },
            Cage {
                sign: Operator::Add,
                target: 7,
                coords: vec![(2, 0), (3, 0), (2, 1)],
            },
            Cage {
                sign: Operator::Subtract,
                target: 2,
                coords: vec![(2, 2), (2, 3)],
            },
            Cage {
                sign: Operator::Add,
                target: 5,
                coords: vec![(2, 5), (3, 5)],
            },
            Cage {
                sign: Operator::Subtract,
                target: 1,
                coords: vec![(3, 1), (3, 2)],
            },
            Cage {
                sign: Operator::Divide,
                target: 3,
                coords: vec![(3, 3), (3, 4)],
            },
            Cage {
                sign: Operator::None,
                target: 3,
                coords: vec![(4, 0)],
            },
            // FIXME: Continue
        ];

        todo!("Continue the cages");
        assert_eq!(validate::all(&cages, &grid.solve(&cages).unwrap()), Valid);
    }
}
