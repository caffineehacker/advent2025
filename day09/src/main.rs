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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Hash)]
struct Input {
    values: Vec<Position>,
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
    let mut best = 0;
    // Find the rectangle for each pair naively for fun
    for i in 0..(input.values.iter().len() - 1) {
        for j in (i + 1)..input.values.iter().len() {
            let a = input.values[i];
            let b = input.values[j];

            let region = ((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1);
            if region > best {
                best = region;
            }
        }
    }

    best
}

fn is_valid_vertical(x: i64, y1: i64, y2: i64, lines: &Vec<(Position, Position)>) -> bool {
    let min_y = y1.min(y2);
    let max_y = y1.max(y2);

    let lines_across_y = lines
        .iter()
        .filter(|(pa, pb)| pa.x.min(pb.x) <= x && pa.x.max(pb.x) >= x && pb.x != pa.x)
        .filter(|(pa, _pb)| pa.y != min_y && pa.y != max_y)
        .filter(|(pa, _pb)| pa.y < max_y && pa.y > min_y)
        .sorted_by_key(|(pa, _)| pa.y)
        .collect_vec();

    // If a line is decreasing in the y we are transitioning from outside to inside
    // If a line is increasing in the y we are transitioning from inside to outside

    lines_across_y.iter().all(|(pa, pb)| pa.y > pb.y)
}

fn is_valid_horizontal(y: i64, x1: i64, x2: i64, lines: &Vec<(Position, Position)>) -> bool {
    let min_x = x1.min(x2);
    let max_x = x1.max(x2);

    let lines_across_x = lines
        .iter()
        .filter(|(pa, pb)| pa.y.min(pb.y) <= y && pa.y.max(pb.y) >= y && pb.y != pa.y)
        .filter(|(pa, _pb)| pa.x != min_x && pa.x != max_x)
        .filter(|(pa, _pb)| pa.x < max_x && pa.x > min_x)
        .sorted_by_key(|(pa, _)| pa.x)
        .collect_vec();

    // If a line is decreasing in the x we are transitioning from inside to outside
    // If a line is increasing in the x we are transitioning from outside to inside

    lines_across_x.iter().all(|(pa, pb)| pa.x < pb.x)
}

fn is_valid(a: &Position, b: &Position, lines: &Vec<(Position, Position)>) -> bool {
    let min_x = a.x.min(b.x);
    let max_x = a.x.max(b.x);
    let min_y = a.y.min(b.y);
    let max_y = a.y.max(b.y);

    is_valid_vertical(min_x, min_y, max_y, lines)
        && is_valid_vertical(max_x, min_y, max_y, lines)
        && is_valid_horizontal(min_y, min_x, max_x, lines)
        && is_valid_horizontal(max_y, min_x, max_x, lines)
}

fn make_lines(input: &Input) -> Vec<(Position, Position)> {
    let mut lines = input
        .values
        .iter()
        .tuple_windows()
        .map(|(a, b)| (*a, *b))
        .collect_vec();
    lines.push((*input.values.last().unwrap(), input.values[0]));
    lines
}

fn part2(input: &Input) -> i64 {
    // Make pairs of points to make searching easier
    let lines = make_lines(input);

    // I used visual analysis of the points to identify that the rectangle must start at either 94800,50143 or 94800,48628

    let mut regions = Vec::new();
    let a = Position { x: 94800, y: 50143 };
    for j in 0..input.values.iter().len() {
        let b = input.values[j];
        if b.y < 50143 {
            continue;
        }

        if !is_valid(&a, &b, &lines) {
            continue;
        }

        let region = ((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1);
        regions.push((region, a, b));
    }

    let a = Position { x: 94800, y: 48628 };
    for j in 0..input.values.iter().len() {
        let b = input.values[j];
        if b.y < 50143 {
            continue;
        }

        if !is_valid(&a, &b, &lines) {
            continue;
        }

        let region = ((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1);
        regions.push((region, a, b));
    }

    regions.sort_by_key(|r| r.0);
    regions.reverse();

    regions[0].0
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
            .map(|line| {
                let point = line.split_once(',').unwrap();
                Position {
                    x: point.0.parse::<i64>().unwrap(),
                    y: point.1.parse::<i64>().unwrap(),
                }
            })
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

        assert_eq!(result1, 50);
    }

    #[test]
    fn test_part2_horizontal() {
        let input = Input {
            values: vec![
                Position { x: 10, y: 10 },
                Position { x: 20, y: 10 },
                Position { x: 20, y: 20 },
                Position { x: 10, y: 20 },
            ],
        };
        let lines = make_lines(&input);

        assert_eq!(is_valid_horizontal(10, 10, 11, &lines), true);
        assert_eq!(is_valid_horizontal(10, 10, 21, &lines), false);
        assert_eq!(is_valid_horizontal(10, 10, 20, &lines), true);
    }
}
