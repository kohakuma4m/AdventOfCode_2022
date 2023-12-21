# Advent of Code 2022 solutions (rust 1.64.0)
- https://adventofcode.com/2022

# To create new day solution, run
  - `cargo run -- generate [dayXX]`
  - This will create next day or `dayXX` solution folder with `templates/default.rs` base file and empty text and test input files
  - Already existing solutions folder will be left untouched
  ## Note
  - Since used module must be known at compile time, you will still have to manually uncomment those lines
    1. `pub mod dayXX;` in `lib.rs`
    2. `"dayXX-1" => solutions::dayXX::solution1(data),` in `main.rs`
    3. `"dayXX-2" => solutions::dayXX::solution2(data),` in `main.rs`

# To run specific day solution, use
  - `cargo run -- dayXX solutionY`
    - `dayXX` is the solution folder
    - `solutionY` is the solution number (e.g: `1`, `2`, etc.)

# To format code with rustfmt, use
  - `cargo fmt`

# To validate solutions against test file(s), use
  - `cargo test [dayXX] [-- --nocapture]`
    - `-- --nocapture` option is for when you want to see logs for passing tests also

# WARNING
  - This is my own personal sandbox. All solutions are work in progress...
  - I tend to revisit my previous solutions as I go along to cleanup my code, but I make no promises ^_^
