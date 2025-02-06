mod cage;
mod orthogonal;
mod tests;

use crate::{Cage, Grid};

#[derive(Clone, Debug)]
pub struct Domain<const N: usize>(pub [[[bool; N]; N]; N]);

impl<const N: usize> Domain<N> {
    fn domain(mut self: Self, grid: &Grid<N>, cages: &[Cage]) -> Self {
        self = self.orthogonal(grid);

        // This prunes the domain of each cage
        for cage in cages {
            self = self.cage(grid, cage);
        }

        self
    }

    #[inline]
    fn iter(self: &Self) -> impl Iterator<Item = &[[bool; N]; N]> {
        self.0.iter()
    }

    #[allow(dead_code)]
    pub(super) fn print(self: &Self) {
        for row in self.iter() {
            for cell in row {
                for val in cell {
                    if *val {
                        print!("1");
                    } else {
                        print!("0");
                    }
                }
                print!("  ");
            }
            println!();
        }
    }

    #[inline]
    fn len(self: &Self) -> usize {
        self.0.len()
    }

    fn count(self: &Self) -> usize {
        let mut count = 0;
        for row in self.iter() {
            for cell in row {
                for val in cell {
                    if *val {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    pub(super) fn new(grid: &Grid<N>, cages: &[Cage]) -> Self {
        let domain = Domain([[[true; N]; N]; N]);
        domain.domain(grid, cages)
    }
}

impl<const N: usize> std::ops::Index<usize> for Domain<N> {
    type Output = [[bool; N]; N];

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const N: usize> std::ops::IndexMut<usize> for Domain<N> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
