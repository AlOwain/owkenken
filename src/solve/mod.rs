mod tests;

use crate::{
    validate::{self, State},
    Cage, Domain, Grid,
};

#[derive(Debug)]
pub struct Move {
    pub val: u8,
    pub pos: (usize, usize),
}

impl<const N: usize> Grid<N> {
    pub fn next_move(self: &Self, domain: &Domain<N>, _cages: &[Cage]) -> Option<Move> {
        let mut next: Option<Move> = None;

        for x in 0..N {
            for y in 0..N {
                if self[x][y] != 0 {
                    continue;
                }

                let first_true = match domain[x][y].iter().position(|&v| v) {
                    Some(v) => v as u8,
                    None => continue,
                };

                match next {
                    Some(old)
                        if domain[x][y].iter().filter(|&&b| b).count()
                            < domain[old.pos.0][old.pos.1].iter().filter(|&&b| b).count() =>
                    {
                        next = Some(Move {
                            val: first_true + 1,
                            pos: (x, y),
                        });
                    }
                    None => {
                        next = Some(Move {
                            val: first_true + 1,
                            pos: (x, y),
                        });
                    }
                    _ => continue,
                }
            }
        }

        next
    }

    pub fn solve(mut self, cages: &[Cage]) -> Option<Self> {
        let mut steps: Vec<(Move, Domain<N>)> = Vec::new();
        let mut domain: Domain<N>;
        let mut backtrack = false;
        loop {
            if backtrack {
                let prev;
                (prev, domain) = steps.pop()?;
                self[prev.pos.0][prev.pos.1] = 0;
                domain[prev.pos.0][prev.pos.1][prev.val as usize - 1] = false;
                backtrack = false;
            } else {
                domain = Domain::new(&self, cages);
            }

            match validate::all(cages, &self) {
                State::Valid => return Some(self),
                State::Unsatisfiable => {
                    backtrack = true;
                    continue;
                }
                _ => (),
            };

            let mv = match self.next_move(&domain, cages) {
                Some(mv) => mv,
                None => {
                    backtrack = true;
                    continue;
                }
            };

            self[mv.pos.0][mv.pos.1] = mv.val;
            steps.push((mv, domain));
        }
    }
}
