use clap::Parser;
use itertools::Itertools;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::{
    collections::{BinaryHeap, HashSet, VecDeque},
    fmt::format,
    fs::File,
    io::{BufRead, BufReader},
};
use z3::ast::Int;
use z3::{Optimize, Solver};

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
    joltage_target: Vec<i64>,
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

fn get_machine_presses_part1(machine: &Machine) -> i64 {
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
        sum += get_machine_presses_part1(machine);
    }

    sum
}

fn get_machine_presses_part2(machine: &Machine) -> i64 {
    // This is not an optimal solution, but it should run in less than half an hour
    let mut seen_states = HashSet::new();
    let mut states_to_process = VecDeque::new();
    let mut starting_state = Vec::new();
    starting_state.resize(machine.joltage_target.len(), 0);
    states_to_process.push_back((0, starting_state));
    let progress = indicatif::ProgressBar::new_spinner();
    let target_indexes_in_increasing_order = machine
        .joltage_target
        .iter()
        .enumerate()
        .sorted_by_key(|(_index, target)| **target)
        .map(|(index, _)| index)
        .collect_vec();

    loop {
        let (moves, start) = states_to_process.pop_front().unwrap();
        if seen_states.contains(&start) {
            continue;
        }
        seen_states.insert(start.clone());
        progress.set_message(format!(
            "{:?}: {:?}   {} left",
            machine.joltage_target,
            start,
            states_to_process.len()
        ));

        if machine.joltage_target == start {
            return moves;
        }

        // See if we can multiply this to get to the end
        if start[0] > 0 && machine.joltage_target[0] % start[0] == 0 {
            let divisor = machine.joltage_target[0] / start[0];
            let mut is_multipliable = true;
            for i in 1..machine.joltage_target.len() {
                if start[i] == 0
                    || machine.joltage_target[i] % start[i] != 0
                    || machine.joltage_target[i] / start[i] != divisor
                {
                    is_multipliable = false;
                    break;
                }
            }

            if is_multipliable {
                let total_moves = moves * divisor;
                return total_moves;
            }
        }

        for i in target_indexes_in_increasing_order.iter().cloned() {
            if start[i] != machine.joltage_target[i] {
                for button in machine.buttons.iter() {
                    if !button.light_indexes.contains(&i) {
                        continue;
                    }
                    let mut modified = start.clone();
                    let mut is_valid = true;
                    for light in button.light_indexes.iter() {
                        modified[*light] += 1;
                        if modified[*light] > machine.joltage_target[*light] {
                            is_valid = false;
                            break;
                        }
                    }
                    if is_valid {
                        states_to_process.push_back(((moves + 1), modified));
                    }
                }
                break;
            }
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    best_case_total_cost: i64,
    cost: i64,
    indicators: Vec<i64>,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .best_case_total_cost
            .cmp(&self.best_case_total_cost)
            .then_with(|| self.cost.cmp(&other.cost))
            .then_with(|| self.indicators.cmp(&other.indicators))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_machine_presses_part2_optimized(machine: &Machine) -> i64 {
    // Let's use z3
    let solver = Optimize::new();

    // One int for each button
    let button_ints = machine
        .buttons
        .iter()
        .enumerate()
        .map(|(i, button)| (button, Int::new_const("btn".to_string() + &i.to_string())))
        .collect_vec();

    let mut buttons_sum: Int = button_ints[0].1.clone();
    for i in 1..button_ints.len() {
        buttons_sum = buttons_sum + &button_ints[i].1;
    }
    solver.minimize(&buttons_sum);
    for button in button_ints.iter() {
        solver.assert(&button.1.ge(0));
    }

    for indicator in 0..machine.joltage_target.len() {
        let buttons_involved = button_ints
            .iter()
            .filter(|(b, _i)| b.light_indexes.contains(&indicator))
            .map(|b| &b.1)
            .collect_vec();
        if buttons_involved.is_empty() {
            println!("Unexpected!");
            continue;
        }

        let mut total: Int = buttons_involved[0].clone();
        for i in 1..buttons_involved.len() {
            total = total + buttons_involved[i];
        }
        solver.assert(&total.eq(machine.joltage_target[indicator]));
    }
    solver.check(&[]);
    let model = solver.get_model().unwrap();

    model.eval(&buttons_sum, true).unwrap().as_i64().unwrap()
}

fn part2(input: &Input) -> i64 {
    input
        .values
        .iter()
        .map(|machine| get_machine_presses_part2_optimized(machine))
        .sum::<i64>()
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
                let joltage_target = parts
                    .last()
                    .unwrap()
                    .trim_start_matches("{")
                    .trim_end_matches("}")
                    .split(",")
                    .map(|j| j.parse::<i64>().unwrap())
                    .collect_vec();

                Machine {
                    lights_count: target.len() as i64,
                    target,
                    buttons,
                    joltage_target,
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

        assert_eq!(result2, 33);
    }
}
