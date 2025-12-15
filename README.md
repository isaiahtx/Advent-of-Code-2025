# 2024 Advent of Code solutions written in Rust

(Plus implementation of some standard data structures + algorithms in Rust)

My code for each day can be found in `src/days`. Code for DSA implementations can be found in `src/`.

To run the code, simply buildthe Rust project and run `cargo run -- <day_number> <part_number> <input_path>`. The final argument is optional; if no input path is provided then the program will default to the file located at `inputs/inputX.txt`, where `X` is the same as the indicated day.

For example, to run my solution to part 2 on day 4 with the file `input.txt`, one would use the command
```
cargo run -- 4 2 input.txt
```

## Unit Tests/CI

[![CI](https://github.com/isaiahtx/Advent-of-Code-2024/actions/workflows/ci.yml/badge.svg)](https://github.com/isaiahtx/Advent-of-Code-2024/actions/workflows/ci.yml/)

Each implemented data structure, algorithm, and daily solution has dedicated unit tests, which are automatically run via a GitHub Actions workflow
