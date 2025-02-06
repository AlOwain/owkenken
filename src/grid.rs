#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grid<const N: usize>(pub [[u8; N]; N]);

impl<const N: usize> Grid<N> {
    #[inline]
    pub fn len(self: &Self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &[u8; N]> {
        self.0.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut [u8; N]> {
        self.0.iter_mut()
    }
}

impl<const N: usize> std::ops::Index<usize> for Grid<N> {
    type Output = [u8];

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const N: usize> std::ops::IndexMut<usize> for Grid<N> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
