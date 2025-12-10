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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Button {
    light_indexes: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Machine {
    lights_count: i64,
    target: Vec<bool>,
    buttons: Vec<Button>,
}

#[derive(Debug, Clone, Hash)]
struct Input {
    values: Vec<Machine>,
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

fn get_machine_presses(machine: &Machine) -> i64 {
    let mut seen_states = HashSet::new();
    let mut states_to_process = VecDeque::new();
    let mut starting_state = Vec::new();
    starting_state.resize(machine.lights_count as usize, false);
    states_to_process.push_back((0, starting_state));

    loop {
        let (moves, start) = states_to_process.pop_front().unwrap();
        if seen_states.contains(&start) {
            continue;
        }
        seen_states.insert(start.clone());

        if machine.target == start {
            return moves;
        }

        for button in machine.buttons.iter() {
            let mut modified = start.clone();
            for light in button.light_indexes.iter() {
                modified[*light] = !modified[*light];
            }
            states_to_process.push_back(((moves + 1), modified));
        }
    }
}

fn part1(input: &Input) -> i64 {
    let mut sum = 0;

    for machine in input.values.iter() {
        sum += get_machine_presses(machine);
    }

    sum
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
                // Line looks like [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
                let parts = line.split_ascii_whitespace().collect_vec();
                let target = parts[0]
                    .trim_start_matches('[')
                    .trim_end_matches(']')
                    .chars()
                    .map(|c| if c == '.' { false } else { true })
                    .collect_vec();

                let mut buttons = Vec::new();
                for i in 1..(parts.len() - 1) {
                    buttons.push(Button {
                        light_indexes: parts[i]
                            .trim_start_matches("(")
                            .trim_end_matches(")")
                            .split(',')
                            .map(|light| light.parse::<usize>().unwrap())
                            .collect_vec(),
                    });
                }

                Machine {
                    lights_count: target.len() as i64,
                    target,
                    buttons,
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

        assert_eq!(result1, 7);
    }

    #[test]
    fn test_part2() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result2 = part2(&input);

        assert_eq!(result2, 0);
    }
}
