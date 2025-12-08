use clap::Parser;
use itertools::Itertools;
use std::{
    collections::HashMap,
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
    z: i64,
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

    let result1 = part1(&input, 1000);
    println!("Part1: {}", result1);

    println!("Part 2: {}", part2(&input))
}

fn part1(input: &Input, iterations: i64) -> i64 {
    // Find the two closest values and connect them
    // They connected together values form a circuit
    // After doing iterations number of connections, find the circuits
    // Multiply together the sizes of the circuits to get the answer

    let mut circuits: Vec<Vec<Position>> = Vec::new();

    // Step 1: Find distances between all points
    let mut distances: HashMap<(Position, Position), f64> = HashMap::new();

    for i in 0..(input.values.len() - 1) {
        for j in (i + 1)..input.values.len() {
            let a = input.values[i];
            let b = input.values[j];
            let distance =
                (((a.x - b.x).pow(2) + (a.y - b.y).pow(2) + (a.z - b.z).pow(2)) as f64).sqrt();
            distances.insert((a, b), distance as f64);
        }
    }

    for distance in distances
        .values()
        .cloned()
        .sorted_by(f64::total_cmp)
        .take(iterations as usize)
    {
        let points = distances.iter().find(|(p, d)| **d == distance).unwrap().0;
        let point_a = points.0;
        let point_b = points.1;

        let circuit_a = circuits
            .iter()
            .find_position(|c| c.iter().any(|p| *p == point_a))
            .map(|p| p.0);
        let circuit_b = circuits
            .iter()
            .find_position(|c| c.iter().any(|p| *p == point_b))
            .map(|p| p.0);

        if circuit_a.is_none() && circuit_b.is_none() {
            circuits.push(vec![point_a, point_b]);
        } else if circuit_a.is_none() {
            circuits[circuit_b.unwrap()].push(point_a);
        } else if circuit_b.is_none() {
            circuits[circuit_a.unwrap()].push(point_b);
        } else {
            // Both are populated so we need to merge the circuits. This leaves the circuit in the vec, but it'll be empty
            let to_remove = circuit_a.unwrap().max(circuit_b.unwrap());
            let to_update = circuit_a.unwrap().min(circuit_b.unwrap());
            if to_remove != to_update {
                let mut removed_circuit = circuits.remove(to_remove);
                circuits
                    .get_mut(to_update)
                    .unwrap()
                    .append(&mut removed_circuit);
            }
        }
    }

    circuits
        .iter()
        .sorted_by_key(|c| c.len())
        .rev()
        .take(3)
        .fold(1, |a, b| a * b.len() as i64)
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

    Input {
        values: lines
            .iter()
            .map(|line| {
                line.split(',')
                    .map(|v| v.parse::<i64>().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .map(|(x, y, z)| Position { x, y, z })
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
        let result1 = part1(&input, 10);

        assert_eq!(result1, 40);
    }

    #[test]
    fn test_part2() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result2 = part2(&input);

        assert_eq!(result2, 0);
    }
}
