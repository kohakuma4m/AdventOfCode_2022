use std::env;
use std::error::Error;
use std::process;

use aoc2022::solutions;
use aoc2022::{create_solution_dir, create_solution_files, find_next_solution_dir, is_solution_dir, read_file, Script, ScriptConfig};

fn main() {
    // Parsing arguments
    let config = ScriptConfig::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments! {err}\n");
        eprintln!("USAGE: cargo run -- (generate|dayXX) [solutionY]");
        process::exit(1);
    });

    // Running scripts
    let result = match config.script_type {
        Script::Generate => generate_next_day_solution(config),
        Script::Run => run_day_solution(config)
    };

    match result {
        Ok(()) => println!("Done !"),
        Err(err) => eprintln!("Error: {err}")
    }
}

/////////////////////////////////////////////////

fn generate_next_day_solution(config: ScriptConfig) -> Result<(), Box<dyn Error>> {
    let folder = match config.solution_folder {
        Some(folder) => folder,
        None => match find_next_solution_dir() {
            Ok(result) => result.unwrap_or(String::from("day01")),
            Err(_) => String::from("day01")
        }
    };

    if folder == "day26" {
        return Ok(()); // No more solution
    }

    if is_solution_dir(&folder) {
        return Ok(()); // Solution folder already exists
    }

    match create_solution_dir(&folder) {
        Ok(()) => println!("Created \"{folder}\" solution dir"),
        Err(err) => {
            eprintln!("Failed to create solution dir: {err}");
            return Err(err);
        }
    }

    match create_solution_files(&folder) {
        Ok(()) => println!("Created \"{folder}\" solution files"),
        Err(err) => {
            eprintln!("Failed to create solution files: {err}");
            return Err(err);
        }
    }

    Ok(())
}

fn run_day_solution(config: ScriptConfig) -> Result<(), Box<dyn Error>> {
    let folder = config.solution_folder.unwrap();
    let solution_key = format!("{}-{}", folder, config.solution_number.unwrap());

    if !is_solution_dir(&folder) {
        // Solution folder does not exists yet
        let error: Result<Result<(), Box<dyn Error>>, &str> = Err("Non existing solution \"{folder}\"");
        return error?;
    }

    // Reading input file
    let input_file = format!("./src/solutions/{}/input.txt", folder);
    let data = read_file(&input_file)?;

    // Running solution
    match &solution_key[..] {
        "day01-1" => {
            solutions::day01::solution1(data);
        },
        "day01-2" => {
            solutions::day01::solution2(data);
        },
        "day02-1" => {
            solutions::day02::solution1(data);
        },
        "day02-2" => {
            solutions::day02::solution2(data);
        },
        "day03-1" => {
            solutions::day03::solution1(data);
        },
        "day03-2" => {
            solutions::day03::solution2(data);
        },
        "day04-1" => {
            solutions::day04::solution1(data);
        },
        "day04-2" => {
            solutions::day04::solution2(data);
        },
        "day05-1" => {
            solutions::day05::solution1(data);
        },
        "day05-2" => {
            solutions::day05::solution2(data);
        },
        "day06-1" => {
            solutions::day06::solution1(data);
        },
        "day06-2" => {
            solutions::day06::solution2(data);
        },
        "day07-1" => {
            solutions::day07::solution1(data);
        },
        "day07-2" => {
            solutions::day07::solution2(data);
        },
        "day08-1" => {
            solutions::day08::solution1(data);
        },
        "day08-2" => {
            solutions::day08::solution2(data);
        },
        "day09-1" => {
            solutions::day09::solution1(data);
        },
        "day09-2" => {
            solutions::day09::solution2(data);
        },
        "day10-1" => {
            solutions::day10::solution1(data);
        },
        "day10-2" => {
            solutions::day10::solution2(data);
        },
        "day11-1" => {
            solutions::day11::solution1(data);
        },
        "day11-2" => {
            solutions::day11::solution2(data);
        },
        "day12-1" => {
            solutions::day12::solution1(data);
        },
        "day12-2" => {
            solutions::day12::solution2(data);
        },
        "day13-1" => {
            solutions::day13::solution1(data);
        },
        "day13-2" => {
            solutions::day13::solution2(data);
        },
        "day14-1" => {
            solutions::day14::solution1(data);
        },
        "day14-2" => {
            solutions::day14::solution2(data);
        },
        "day15-1" => {
            solutions::day15::solution1(data);
        },
        "day15-2" => {
            solutions::day15::solution2(data);
        },
        "day16-1" => {
            solutions::day16::solution1(data);
        },
        "day16-2" => {
            solutions::day16::solution2(data);
        },
        "day17-1" => {
            solutions::day17::solution1(data);
        },
        "day17-2" => {
            solutions::day17::solution2(data);
        },
        "day18-1" => {
            solutions::day18::solution1(data);
        },
        "day18-2" => {
            solutions::day18::solution2(data);
        },
        "day19-1" => {
            solutions::day19::solution1(data);
        },
        "day19-2" => {
            solutions::day19::solution2(data);
        },
        "day20-1" => {
            solutions::day20::solution1(data);
        },
        "day20-2" => {
            solutions::day20::solution2(data);
        },
        "day21-1" => {
            solutions::day21::solution1(data);
        },
        "day21-2" => {
            solutions::day21::solution2(data);
        },
        "day22-1" => {
            solutions::day22::solution1(data);
        },
        "day22-2" => {
            solutions::day22::solution2(data);
        },
        "day23-1" => {
            solutions::day23::solution1(data);
        },
        "day23-2" => {
            solutions::day23::solution2(data);
        },
        "day24-1" => {
            solutions::day24::solution1(data);
        },
        "day24-2" => {
            solutions::day24::solution2(data);
        },
        "day25-1" => {
            solutions::day25::solution1(data);
        },
        "day25-2" => {
            solutions::day25::solution2(data);
        },
        _ => ()
    }

    Ok(())
}
