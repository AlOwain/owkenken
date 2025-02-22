# OwKenKen
> excuse the casing, I dislike it too, it's just what KenKen does.

## Optimization Ideas

- Memoization
  - According to (Michael Heyeck)[http://www.mlsite.net/neknek/], there is something to be memoized; although I can not figure out exactly what it is.
- Bitvecs
  - The domain could be a bitmap
- Parallelism
  - Multithreading is immediately applicable via recursion.
- Naïvete
  - Build upon the domain of the earlier move, if a move did not work for the parent move, then it won't work for the child.
  - Instead of recalculating the entire domain at each move, you can invalidate rows or columns.
  - Use MRV with LCV for the value

# Notes

- There are orders of magnitude more moves tried on a grid than there are possible moves in the search space, which means:
  1. there are moves that are being tried more than once;
  2. since it is is finding a solution in the end, it means that it is not completely cyclic;
  3. consider that the following move (idx: (0, 0) val: 1) has been tried at the start, any moves later on should account that
      the move will not work for them;
  4. since if a move was invalid before you, it could never be valid for you, since you were explored as a choice with it, and;
  5. we are not accounting for that in the domain, which is why different grids are being tried out in different ordering.
- Benchmark the effect of using a u16 instead of a u8
  - Cache misses are probably higher
  - Memory required is higher
  - Conversion between the types probably costs a bit more
- To compare any two domain pruning algorithms
  - The more inclusive one will always be slower,
  - The correctness of the exclusive algorithm can be proven by checking if the problem is:
    1. Still solvable
    2. If not solvable then the difference in the domain must not exclude any valid states.
- Use `rstest` to simplify the tests
- Use `criterion` to create benchmarks
- Look into the speed of indexing the grid using `[(usize, usize)]` and `[usize] -> slice -> [usize]`.
