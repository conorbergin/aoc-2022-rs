# Advent of Code 2022 in Rust

This is the third year I have attempted AoC, and the first time I have finished it. Last year I used Julia, which I think is actually pretty close to perfect for this sort of thing, it has an REPL, linear algebra in the standard library and a clever JIT compiler that can produce code about as performant as C, C++ and Rust, much more convieniently. This year I wanted to write readable, performant, and robust code, and Rust seems to me the best language for that job. Did I succeed? No. Some of my solutions are nice, some are really not. I didn't do much clean up work, once I reached the mid forties and was grinding out some of the harder problems and trying to figure out why some of the ones I had given up on earlier were working for the sample input but not the real one, my enthusiam really started to wane.

## Rust

Rust is really great, as long as you don't write too much of it. I didn't have many problems with the borrow checker becuase I didn't really allocate much.

- I used itertools a lot, the iproduct! macro in particular to avoid writing nested for loops.
- 

*Highlights:*
- Day 22: I came up with an algorithm that uses a stack to match edges of the cube net.
- Day 25: Instead of translating sanfu into base 10 and back again, I implemented adding in snafu itself

*Lowlights:*
- Day 21: For part 1 I used `replace()` to make one long equation in a single sting and then ran it through an algebra solver I found. Part 2 was not going to be so simple, so like a real programmer, I plugged a number into the equation and checked to solution, then using manual gradient descent, after about 50 iterations, I got the right 13 digit number.

