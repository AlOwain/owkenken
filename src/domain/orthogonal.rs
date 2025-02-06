use super::Domain;
use crate::Grid;

impl<const N: usize> Domain<N> {
    pub(super) fn orthogonal(mut self, grid: &Grid<N>) -> Self {
        let n = grid.len();
        for x in 0..n {
            for y in 0..n {
                if grid[x][y] != 0 {
                    for i in 0..n {
                        // This removes all the other values in taken positions
                        self[x][y][i] = false;
                        // This removes all elements of the same value from...
                        self[x][i][grid[x][y] as usize - 1] = false; // Each column
                        self[i][y][grid[x][y] as usize - 1] = false; // Each row
                    }
                }
            }
        }
        self
    }
}
