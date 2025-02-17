mod tests;

use crate::{
    validate::{self, State},
    Cage, Domain, Grid,
};

#[derive(Debug)]
pub struct Move {
    pub val: u8,
    pub pos: (u8, u8),
}

impl<const N: usize> Grid<N> {
    pub fn next_move(self: &Self, domain: &Domain<N>) -> Option<Move> {
        let mut next: Option<Move> = None;

        for x in 0..N {
            for y in 0..N {
                if self[x][y] != 0 {
                    continue;
                }

                match next {
                    Some(ref old)
                        if domain[x][y].iter().filter(|&&b| b).count()
                            < domain[old.pos.0 as usize][old.pos.1 as usize]
                                .iter()
                                .filter(|&&b| b)
                                .count() =>
                    {
                        ()
                    }
                    None => (),
                    _ => continue,
                }

                let val = domain[x][y].iter().position(|&v| v).map(|v| v as u8);
                let val = match val {
                    Some(val) => val,
                    None => continue,
                };

                next = Some(Move {
                    val,
                    pos: (x as u8, y as u8),
                });
            }
        }

        next
    }

    pub fn solve(mut self, cages: &[Cage]) -> Option<Self> {
        let mut steps: Vec<((u8, u8), Domain<N>)> = Vec::new();
        let mut domain: Domain<N>;
        let mut backtrack = false;
        loop {
            if backtrack {
                let prev;
                (prev, domain) = steps.pop()?;
                self[prev.0 as usize][prev.1 as usize] = 0;
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

            let mv = match self.next_move(&domain) {
                Some(mv) => mv,
                None => {
                    backtrack = true;
                    continue;
                }
            };

            self[mv.pos.0 as usize][mv.pos.1 as usize] = mv.val + 1;
            domain[mv.pos.0 as usize][mv.pos.1 as usize][mv.val as usize] = false;
            steps.push((mv.pos, domain));
        }
    }
}
