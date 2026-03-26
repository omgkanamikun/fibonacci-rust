## Fibonacci Sequence Generator 🦀

* [General info](#general-info)
* [How it works](#how-it-works)
* [Features](#features)
* [Requirements](#requirements)
* [Usage](#usage)
* [License](#license)

## General info

A small Rust command-line program that prints the first `N` Fibonacci numbers.  
It reads the sequence length from a standard input, validates the input, and then generates the sequence with a recursive
Fibonacci function backed by a cache.

## How it works

The program:

1. asks for a sequence length
2. keeps prompting until the input is a valid positive integer
3. calculates Fibonacci numbers using memoization
4. prints the sequence one number at a time

The implementation uses `Option<usize>` for caching, which avoids the old sentinel-value bug where `0` could collide
with `fib(0)`.

## Features

- Interactive command-line interface
- Input validation for sequence length
- Memoized Fibonacci calculation
- Overflow protection with a clear panic if a value exceeds `usize`
- Clear output formatting
- Tests for parsing and Fibonacci behavior

## Requirements

- Rust 1.70.0 or higher

## Usage

Run the program with the following command:

```bash
$ cargo run --release
```

Then enter the desired sequence length when prompted.

## Notes

- The program prints `0` as the first Fibonacci number.
- Huge inputs may still overflow `usize`.
- The current implementation is recursive, so extremely large inputs can still be expensive.

## License

This project is open source and available under the MIT License.