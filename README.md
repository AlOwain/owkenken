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
  - Instead of recalculating the entire domain at each move, you can invalidate rows or columns.

# Notes

- See the effect of using a u16 instead of a u8
  - Cache misses are probably higher
  - Memory required is higher
  - Conversion between the types probably costs a bit more
