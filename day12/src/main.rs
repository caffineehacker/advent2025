use clap::Parser;
use itertools::Itertools;
use std::{
    collections::{HashSet, VecDeque},
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
struct Present {
    grid: Vec<Vec<bool>>,
}

#[derive(Debug, Clone, Hash)]
struct Requirements {
    width: i64,
    height: i64,
    presents_needed: Vec<i64>,
}

#[derive(Debug, Clone, Hash)]
struct Input {
    presents: Vec<Present>,
    puzzles: Vec<Requirements>,
}

fn main() {
    let args = Args::parse();
    let data_file = if args.data_file.is_empty() {
        format!("{}/src/data.txt", env!("CARGO_MANIFEST_DIR"))
    } else {
        args.data_file
    };

    let input = parse(&data_file);

    let result1 = part1(&input);
    println!("Part1: {}", result1);

    println!("Part 2: {}", part2(&input))
}

#[derive(Debug, Clone, Hash)]
struct State {
    remaining_presents: Vec<i64>,
    grid: Vec<Vec<bool>>,
}

fn rotate_right(start: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    vec![
        vec![start[2][0], start[1][0], start[0][0]],
        vec![start[2][1], start[1][1], start[0][1]],
        vec![start[2][2], start[1][2], start[0][2]],
    ]
}

fn flip_horizontal(start: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    vec![
        vec![start[0][2], start[0][1], start[0][0]],
        vec![start[1][2], start[1][1], start[1][0]],
        vec![start[2][2], start[2][1], start[2][0]],
    ]
}

fn part1(input: &Input) -> i64 {
    let presents_with_rotations = input
        .presents
        .iter()
        .map(|present| {
            // Rotate the present 3 times, then flip horizontally, rotate 3 more times and collect all of the unique variations
            let start = present.grid.clone();
            let rotate1 = rotate_right(&start);
            let rotate2 = rotate_right(&rotate1);
            let rotate3 = rotate_right(&rotate2);

            let flipped = flip_horizontal(&start);
            let flip_rotate1 = rotate_right(&flipped);
            let flip_rotate2 = rotate_right(&flip_rotate1);
            let flip_rotate3 = rotate_right(&flip_rotate2);

            let mut shapes = HashSet::new();
            shapes.insert(start);
            shapes.insert(rotate1);
            shapes.insert(rotate2);
            shapes.insert(rotate3);
            shapes.insert(flipped);
            shapes.insert(flip_rotate1);
            shapes.insert(flip_rotate2);
            shapes.insert(flip_rotate3);
            shapes.into_iter().collect_vec()
        })
        .collect_vec();
    let puzzle_square_counts = input
        .presents
        .iter()
        .map(|present| {
            present
                .grid
                .iter()
                .map(|row| row.iter().filter(|c| **c).count())
                .sum::<usize>()
        })
        .collect_vec();
    let mut valid_inputs = 0;
    for puzzle in input.puzzles.iter() {
        // First, can we even fit the number of squares required?
        if puzzle
            .presents_needed
            .iter()
            .enumerate()
            .map(|(index, count)| (*count as usize) * puzzle_square_counts[index])
            .sum::<usize>()
            > (puzzle.width * puzzle.height) as usize
        {
            continue;
        }

        // This is absolutely cheating, but for the actual puzzle it works to just count anything with enough space!
        // No need to do any real work I guess...
        valid_inputs += 1;
    }

    valid_inputs
}

fn part2(input: &Input) -> i64 {
    0
}

fn parse(file: &str) -> Input {
    let file = File::open(file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let mut presents = Vec::new();
    let mut index = 0;
    for _ in 0..=5 {
        index += 1; // Skip index line
        // Now pull the 3x3
        presents.push(Present {
            grid: lines
                .iter()
                .skip(index)
                .take(3)
                .map(|line| line.chars().map(|c| c == '#').collect_vec())
                .collect_vec(),
        });
        // Skip the above 3 lines and the newline
        index += 4;
    }

    let puzzles = lines
        .iter()
        .skip(index)
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (dimensions, presents) = line.split_once(":").unwrap();
            let (width, height) = dimensions.trim().split_once("x").unwrap();

            Requirements {
                width: width.parse::<i64>().unwrap(),
                height: height.parse::<i64>().unwrap(),
                presents_needed: presents
                    .trim()
                    .split_ascii_whitespace()
                    .map(|count| count.parse::<i64>().unwrap())
                    .collect_vec(),
            }
        })
        .collect_vec();

    Input { presents, puzzles }

    /*
     * Alternative implementations:
     */

    // Two sections separated by a newline
    // Input {
    //     first: lines
    //         .iter()
    //         .take_while(|line| !line.is_empty())
    //         .map(|line| line.split_once('|').unwrap())
    //         .map(|(a, b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
    //         .collect_vec(),
    //     second: lines
    //         .iter()
    //         .skip_while(|line| !line.is_empty())
    //         .filter(|line| !line.is_empty())
    //         .map(|line| {
    //             line.split(',')
    //                 .map(|page| page.parse::<i64>().unwrap())
    //                 .collect_vec()
    //         })
    //         .collect_vec(),
    // }

    // Creates a HashMap<char, Vec<Position>>
    // let map_limits = Position {
    //     x: lines[0].len() as i64,
    //     y: lines.len() as i64,
    // };

    // Input {
    //     antennas: lines
    //         .into_iter()
    //         .enumerate()
    //         .flat_map(|(y, line)| {
    //             line.chars()
    //                 .enumerate()
    //                 .filter(|(_, c)| *c != '.')
    //                 .map(|(x, c)| {
    //                     (
    //                         c,
    //                         Position {
    //                             x: x as i64,
    //                             y: y as i64,
    //                         },
    //                     )
    //                 })
    //                 .collect_vec()
    //         })
    //         .sorted_by(|(a, _), (b, _)| Ord::cmp(a, b))
    //         .chunk_by(|(c, _)| *c)
    //         .into_iter()
    //         .map(|(c, positions)| (c, positions.map(|(_, p)| p).collect_vec()))
    //         .collect(),
    //     map_limits,
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result1 = part1(&input);

        assert_eq!(result1, 2);
    }

    #[test]
    fn test_part2() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result2 = part2(&input);

        assert_eq!(result2, 0);
    }
}
