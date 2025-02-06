use crate::validate::{Cage, Operator};

use super::{Domain, Grid};

impl<const N: usize> Domain<N> {
    fn subtraction(mut self, grid: &Grid<N>, cage: &Cage) -> Self {
        let [(x1, y1), (x2, y2)] = cage.coords[..] else {
            unreachable!()
        };

        let mut new_domain = [[false; N], [false; N]];
        for i in 0..(N - cage.target as usize) {
            let res = i + cage.target as usize;
            if self[x1][y1][i] || grid[x1][y1] == i as u8 + 1 {
                new_domain[1][res] = true;
            }
            if self[x2][y2][i] || grid[x2][y2] == i as u8 + 1 {
                new_domain[0][res] = true;
            }

            if self[x1][y1][res] || grid[x1][y1] == res as u8 + 1 {
                new_domain[1][i] = true;
            }
            if self[x2][y2][res] || grid[x2][y2] == res as u8 + 1 {
                new_domain[0][i] = true;
            }
        }

        for i in 0..N {
            self[x1][y1][i] = self[x1][y1][i] && new_domain[0][i];
            self[x2][y2][i] = self[x2][y2][i] && new_domain[1][i];
        }
        self
    }

    fn division(mut self, grid: &Grid<N>, cage: &Cage) -> Self {
        let [(x1, y1), (x2, y2)] = cage.coords[..] else {
            unreachable!()
        };

        let mut new_domain = [[false; N], [false; N]];
        let mut i = 1;
        while i <= N {
            let j1 = i / cage.target as usize;
            if j1 > 0 {
                let j1 = j1 - 1;
                if self[x1][y1][j1] || grid[x1][y1] == j1 as u8 + 1 {
                    new_domain[1][i - 1] = true;
                }
                if self[x2][y2][j1] || grid[x2][y2] == j1 as u8 + 1 {
                    new_domain[0][i - 1] = true;
                }
            }

            let j2 = (i * cage.target as usize) - 1;
            if j2 < N {
                if self[x1][y1][j2] || grid[x1][y1] == j2 as u8 + 1 {
                    new_domain[1][i - 1] = true;
                }
                if self[x2][y2][j2] || grid[x2][y2] == j2 as u8 + 1 {
                    new_domain[0][i - 1] = true;
                }
            }

            i *= cage.target as usize;
        }

        for i in 0..N {
            self[x1][y1][i] = self[x1][y1][i] && new_domain[0][i];
            self[x2][y2][i] = self[x2][y2][i] && new_domain[1][i];
        }
        self
    }

    pub(super) fn cage(mut self, grid: &Grid<N>, cage: &Cage) -> Self {
        match cage.sign {
            Operator::None => {
                let (x, y) = cage.coords[0];

                for i in 0..N {
                    if i == cage.target as usize - 1 {
                        continue;
                    }
                    self[x][y][i] = false;
                }
                self
            }
            Operator::Subtract => self.subtraction(grid, cage),
            Operator::Divide => self.division(grid, cage),
            // TODO Build addition and multiplication
            _ => self,
        }
    }
}
