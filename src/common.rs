use super::days;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;

pub type LinesIterator = std::io::Lines<std::io::BufReader<std::fs::File>>;

pub fn run_w_args(args: &[String]) -> String {
    if args.len() < 3 || args.len() > 4 {
        eprintln!(
            "Usage: {} <day number> <part 1 or 2> [path (optional)]",
            args[0]
        );
        process::exit(1);
    }

    let day_number: u8 = match args[1].parse() {
        Ok(n) if n > 0 => n,
        _ => {
            eprintln!(
                "Please provide a valid positive integer for the day number"
            );
            process::exit(1);
        }
    };

    let part: u8 = match args[2].parse() {
        Ok(n) if (n == 1) || (n == 2) => n,
        _ => {
            eprintln!("Please provide either 1 or 2 to indicate which part");
            process::exit(1);
        }
    };

    let path: String = if args.len() == 4 {
        args[3].clone()
    } else {
        format!("./inputs/input{day_number}.txt")
    };

    println!("Running part {part} of day {day_number} using input {path}.");
    println!();

    let mut lines: LinesIterator = read_lines(path).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    match part {
        1 => match day_number {
            7 => days::day07::run1(&mut lines),
            _ => panic!("Incomplete day."),
        },
        2 => match day_number {
            7 => days::day07::run2(&mut lines),
            _ => panic!("Incomplete day."),
        },
        _ => {
            panic!("YOU SHOULD NEVER SEE THIS!!!!")
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<LinesIterator>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
