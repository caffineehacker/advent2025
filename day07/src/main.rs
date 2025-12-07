use clap::Parser;
use itertools::Itertools;
use std::{
    cell,
    collections::{HashMap, HashSet},
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
    values: Vec<Vec<char>>,
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

fn part1(input: &Input) -> i64 {
    let width = input.values[0].len();
    let height = input.values.len();
    // S is the start of the beam on the first line
    let start_index: (usize, usize) = (input.values[0].iter().position(|c| *c == 'S').unwrap(), 0);
    let mut beams_to_process: Vec<(usize, usize)> = Vec::new();
    beams_to_process.push(start_index);
    let mut processed_splitters = HashSet::new();

    let mut splits = 0;
    while !beams_to_process.is_empty() {
        let beam = beams_to_process.pop().unwrap();

        if beam.1 < height - 1 {
            let cell = input.values[beam.1][beam.0];
            if cell == '^' {
                if processed_splitters.contains(&beam) {
                    continue;
                }
                if beam.0 > 0 {
                    beams_to_process.push((beam.0 - 1, beam.1));
                }
                if beam.0 < width - 1 {
                    beams_to_process.push((beam.0 + 1, beam.1));
                }
                splits += 1;
                processed_splitters.insert(beam);
            } else {
                beams_to_process.push((beam.0, beam.1 + 1));
            }
        }
    }

    splits
}

fn get_world_count(
    point: (usize, usize),
    input: &Input,
    points_processed: &mut HashMap<(usize, usize), i64>,
) -> i64 {
    if point.1 == input.values.len() {
        return 1;
    }

    if let Some(worlds) = points_processed.get(&point) {
        return *worlds;
    }

    if input.values[point.1][point.0] == '^' {
        let worlds = get_world_count((point.0 - 1, point.1 + 1), input, points_processed)
            + get_world_count((point.0 + 1, point.1 + 1), input, points_processed);
        points_processed.insert(point, worlds);
        return worlds;
    }

    return get_world_count((point.0, point.1 + 1), input, points_processed);
}

fn part2(input: &Input) -> i64 {
    // S is the start of the beam on the first line
    let start: (usize, usize) = (input.values[0].iter().position(|c| *c == 'S').unwrap(), 0);
    let mut points_processed = HashMap::new();

    return get_world_count(start, input, &mut points_processed);
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
            .map(|line| line.chars().collect())
            .collect_vec(),
    }

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

        assert_eq!(result1, 21);
    }

    #[test]
    fn test_part2() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result2 = part2(&input);

        assert_eq!(result2, 40);
    }
}
