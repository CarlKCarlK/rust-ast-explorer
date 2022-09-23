# shortestpaper
## Rust and WASM version of famous shortest paper

In 1769, mathematician Euler made his "sum of powers" conjecture. In 1966, L. J. Lander and T. R. Parkin disproved Euler using a CDC 6600 mainframe computer. Their paper famously contained just two sentences. In 2022, your phone can find their counterexample in about 0.02 seconds. Try it at: https://carlkcarlk.github.io/shortestpaper/ (just press the "Go!" button).

## More Explanation

Disclaimer: This project is just for fun. It is of no mathematical importance.

At one level, this project shows computer progress. A computer program that in 1966 needed a giant computer can now run efficiently inside a web browser, even on a phone.

Most of the stuff you see in your web browser runs a program on a far-away computer rented from Amazon, Microsoft, Google, etc. For this web page, however, your computer or phone or tablet is running the mathematical program. No far-away computer need be rented. You might ask, couldn’t Adobe Flash do that 15 years ago? Yes, it could. The difference now is that this new method (called WASM or WebAssembly) is secure, runs on almost all devices, and is much faster. (It can even [run old Flash games](https://ruffle.rs/), securely!)

As to the mathematical problem, it is kind of like Sudoku. 1769, Euler guessed that no one could ever find five numbers greater than 2, such that a^5+b^5+c^5+d^5=e^5 (where “a^5” means a times itself 5 times). In other words, he defined a puzzle and guessed that it was unsolvable. In 1966, two folks used a computer to just start searching for numbers. They discovered a solution, namely, 27^5+84^5+110^5+133^5=144^5.

The math paper with this solution became famous, not because the puzzle from Euler was important but because the paper was only two sentences long. This project re-creates that famous paper, but now, in a sense, the paper itself finds solutions to Euler’s puzzle.

For a discussion on among Reddit programmers about this project, see https://www.reddit.com/r/rust/comments/sf8mqp/recreating_worlds_shortest_math_paper_with_rust/

## Team
This project was created by the Rust Shortest Paper Team. Members are listed in Reference 3 of the web page: https://carlkcarlk.github.io/shortestpaper/