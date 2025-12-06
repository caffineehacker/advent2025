use clap::Parser;
use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "")]
    data_file: String,
    #[arg(long)]
    debug: bool,
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// struct Position {
//     x: i64,
//     y: i64,
// }

#[derive(Debug, Clone, Hash)]
struct Input {
    values: Vec<Vec<i64>>,
    operations: Vec<char>,
}

#[derive(Debug, Clone, Hash)]
struct Input2 {
    problems: Vec<(Vec<i64>, char)>,
}

fn main() {
    let args = Args::parse();
    let data_file = if args.data_file.is_empty() {
        format!("{}/src/data.txt", env!("CARGO_MANIFEST_DIR"))
    } else {
        args.data_file
    };

    let input = parse(&data_file);
    let input2 = parse2(&data_file);

    let result1 = part1(&input);
    println!("Part1: {}", result1);

    println!("Part 2: {}", part2(&input2))
}

fn part1(input: &Input) -> i64 {
    let mut sum = 0;
    for i in 0..input.operations.len() {
        let numbers = input
            .values
            .iter()
            .map(|numbers| numbers.get(i).unwrap())
            .collect_vec();

        let operation = input.operations.get(i).unwrap();

        if *operation == '+' {
            sum += numbers.iter().cloned().cloned().sum::<i64>();
        } else if *operation == '*' {
            sum += numbers
                .iter()
                .cloned()
                .cloned()
                .reduce(|a, b| a * b)
                .unwrap();
        }
    }

    sum
}

fn part2(input: &Input2) -> i64 {
    let mut sum = 0;
    for problem in input.problems.iter() {
        let numbers = &problem.0;

        let operation = problem.1;

        if operation == '+' {
            sum += numbers.into_iter().cloned().sum::<i64>();
        } else if operation == '*' {
            sum += numbers.into_iter().cloned().reduce(|a, b| a * b).unwrap();
        }
    }

    sum
}

fn parse(file: &str) -> Input {
    let file = File::open(file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    Input {
        values: lines
            .iter()
            .take(lines.len() - 1)
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|v| v.parse::<i64>().unwrap())
                    .collect()
            })
            .collect_vec(),
        operations: lines
            .last()
            .unwrap()
            .split_ascii_whitespace()
            .map(|v| v.chars().take(1).last().unwrap())
            .collect_vec(),
    }
}

fn parse2(file: &str) -> Input2 {
    let file = File::open(file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.expect("Failed to read line").chars().collect_vec())
        .collect();

    let number_rows = lines.len() - 1;

    let mut problems = Vec::new();
    // White space maters here, the last line is the easiest to find the end / start of the next set of numbers
    let mut index = lines[0].len() - 1;
    let mut current_section = Vec::new();
    loop {
        let mut current_column = 0;
        for row in 0..number_rows {
            if lines[row][index] != ' ' {
                current_column = current_column * 10;
                current_column += lines[row][index].to_string().parse::<i64>().unwrap();
            }
        }

        if current_column > 0 {
            current_section.push(current_column);
        }

        if lines[number_rows][index] != ' ' {
            problems.push((current_section, lines[number_rows][index]));
            current_section = Vec::new();
        }

        if index == 0 {
            break;
        }
        index -= 1;
    }

    Input2 { problems: problems }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result1 = part1(&input);

        assert_eq!(result1, 4277556);
    }

    #[test]
    fn test_part2() {
        let input = parse2(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result2 = part2(&input);

        assert_eq!(result2, 3263827);
    }
}
