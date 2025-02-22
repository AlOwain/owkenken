use tracing::{trace, trace_span};

mod tests;

use crate::{
    validate::{self, State},
    Cage, Domain, Grid,
};

type Move = (u8, (u8, u8));

impl<const N: usize> Grid<N> {
    pub fn next_move(self: &Self, domain: &Domain<N>) -> Option<Move> {
        let mut next: Option<Move> = None;

        for x in 0..N {
            for y in 0..N {
                if self[x][y] != 0 {
                    continue;
                }

                let val = domain[x][y].iter().position(|&v| v).map(|v| v as u8)?;

                // What are we doing here exactly? Is it optimal to pick the value with the smallest domain
                match next {
                    Some(ref old)
                        if domain[x][y].iter().filter(|&&b| b).count()
                            < domain[old.1 .0 as usize][old.1 .1 as usize]
                                .iter()
                                .filter(|&&b| b)
                                .count() =>
                    {
                        ()
                    }
                    None => (),
                    _ => continue,
                }

                next = Some((val, (x as u8, y as u8)));
            }
        }

        next
    }

    pub fn solve(mut self, cages: &[Cage]) -> Option<Self> {
        let mut consecutive_moves = 0;
        let mut consecutive_backtracks = 0;
        let mut steps: Vec<((u8, u8), Domain<N>)> = Vec::new();
        let mut domain: Domain<N>;
        let mut backtrack = false;

        let span = trace_span!("solve");
        let mut _guard = span.enter();
        loop {
            if backtrack {
                let prev;
                consecutive_backtracks += 1;

                (prev, domain) = steps.pop()?;
                // TODO: ideally, the domain should be pruned here, right?
                trace!(
                    "Backtracking #{consecutive_backtracks}: moves={consecutive_moves} pos=({x}, {y})",
                    x = prev.0,
                    y = prev.1,
                    consecutive_moves = consecutive_moves,
                    consecutive_backtracks = consecutive_backtracks
                );

                // This drops the previous span guard and creates a new one at each backtrack.
                // This should not be `let _ = ...`, as the span is left when it is dropped.
                // and the `_` name immediately drops it.
                drop(_guard);
                _guard = span.enter();
                consecutive_moves = 0;
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

            consecutive_moves += 1;
            consecutive_backtracks = 0;
            trace!(
                "Making move #{consecutive_moves}: val={val}, pos=({x}, {y})",
                val = mv.0,
                x = mv.1 .0,
                y = mv.1 .1,
            );

            self[mv.1 .0 as usize][mv.1 .1 as usize] = mv.0 + 1;
            domain[mv.1 .0 as usize][mv.1 .1 as usize][mv.0 as usize] = false;
            steps.push((mv.1, domain));
        }
    }
}
