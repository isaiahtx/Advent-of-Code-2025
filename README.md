# 2024 Advent of Code solutions written in Rust

My code for each day can be found in `src/days`. To run the code, simply build
the Rust project and run `cargo run -- <day_number> <part_number>
<input_path>`. The final argument is optional; if no input path is provided
then the program will default to the file located at `inputs/inputX.txt`, where
`X` is the same as the indicated day.

For example, to run my solution to part 2 on day 4 with the file `input.txt`,
one would use the command
```
cargo run -- 4 2 input.txt
```