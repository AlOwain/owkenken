use super::State::{self, *};
use crate::Grid;

pub struct Cage {
    pub sign: Operator,
    pub target: u16,
    pub coords: Vec<(usize, usize)>,
}

pub enum Operator {
    None,
    Add,
    Multiply,
    Subtract,
    Divide,
}

impl Cage {
    fn result<const N: usize>(self: &Self, grid: &Grid<N>) -> State {
        let res: u16 = match self.sign {
            Operator::Add => {
                assert!(self.coords.len() > 1);
                let mut sum = 0u16;
                for &(x, y) in &self.coords {
                    if grid[x][y] == 0 {
                        return Satisfiable;
                    }
                    sum += grid[x][y] as u16;
                }
                sum
            }
            Operator::Multiply => {
                assert!(self.coords.len() > 1);
                let mut product = 1u16;
                for &(x, y) in &self.coords {
                    if grid[x][y] == 0 {
                        return Satisfiable;
                    }
                    product *= grid[x][y] as u16;
                }
                product
            }
            Operator::Subtract => {
                let [(x1, y1), (x2, y2)] = self.coords[..] else {
                    unreachable!()
                };
                if grid[x1][y1] == 0 || grid[x2][y2] == 0 {
                    return Satisfiable;
                }
                grid[x1][y1].abs_diff(grid[x2][y2]) as u16
            }
            Operator::Divide => {
                let [(x1, y1), (x2, y2)] = self.coords[..] else {
                    unreachable!()
                };
                if grid[x1][y1] == 0 || grid[x2][y2] == 0 {
                    return Satisfiable;
                }

                let max = std::cmp::max(grid[x1][y1], grid[x2][y2]);
                let min = std::cmp::min(grid[x1][y1], grid[x2][y2]);

                if max % min != 0 {
                    return State::Unsatisfiable;
                }

                (max / min) as u16
            }
            Operator::None => {
                let [(x, y)] = self.coords[..] else {
                    unreachable!()
                };
                if grid[x][y] == 0 {
                    return Satisfiable;
                }
                grid[x][y] as u16
            }
        };

        if res != self.target {
            return State::Unsatisfiable;
        }
        State::Valid
    }

    pub(crate) fn validate<const N: usize>(self: &Self, grid: &Grid<N>) -> State {
        self.result(grid)
    }
}
