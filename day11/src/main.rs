use clap::Parser;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, AddAssign},
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

#[derive(Debug, Clone)]
struct Input {
    connections: HashMap<String, Vec<String>>,
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
    let mut seen_paths = HashSet::new();
    let mut paths_to_process = Vec::new();
    paths_to_process.push(vec!["you".to_string()]);
    let mut path_count = 0;

    while let Some(path) = paths_to_process.pop() {
        if seen_paths.contains(&path) {
            continue;
        }
        seen_paths.insert(path.clone());

        let last = path.last().unwrap();
        if last == "out" {
            path_count += 1;
            continue;
        }

        for target in &input.connections[last] {
            let mut new_path = path.clone();
            new_path.push(target.clone());
            paths_to_process.push(new_path);
        }
    }

    path_count
}

#[derive(Hash, Clone, Copy, Debug)]
struct TargetPathInfo {
    paths_with_dac: i64,
    paths_with_fft: i64,
    paths_with_both: i64,
    paths_with_neither: i64,
}

impl Default for TargetPathInfo {
    fn default() -> Self {
        Self {
            paths_with_dac: 0,
            paths_with_fft: 0,
            paths_with_both: 0,
            paths_with_neither: 0,
        }
    }
}

impl Add for TargetPathInfo {
    type Output = TargetPathInfo;

    fn add(self, rhs: Self) -> Self::Output {
        TargetPathInfo {
            paths_with_both: self.paths_with_both + rhs.paths_with_both,
            paths_with_dac: self.paths_with_dac + rhs.paths_with_dac,
            paths_with_fft: self.paths_with_fft + rhs.paths_with_fft,
            paths_with_neither: self.paths_with_neither + rhs.paths_with_neither,
        }
    }
}

impl AddAssign for TargetPathInfo {
    fn add_assign(&mut self, rhs: Self) {
        self.paths_with_both += rhs.paths_with_both;
        self.paths_with_dac += rhs.paths_with_dac;
        self.paths_with_fft += rhs.paths_with_fft;
        self.paths_with_neither += rhs.paths_with_neither;
    }
}

fn part2(input: &Input) -> i64 {
    let mut seen_paths = HashSet::new();
    let mut paths_to_process = HashMap::new();
    let mut known_paths: HashMap<String, HashMap<String, TargetPathInfo>> = HashMap::new();
    for machine in input.connections.keys() {
        let mut deque = VecDeque::new();
        deque.push_back(vec![machine.to_string()]);
        paths_to_process.insert(machine.to_string(), deque);
        known_paths.insert(machine.to_string(), HashMap::new());
    }
    known_paths.insert(
        "out".to_string(),
        vec![(
            "out".to_string(),
            TargetPathInfo {
                paths_with_neither: 1,
                ..Default::default()
            },
        )]
        .into_iter()
        .collect(),
    );
    let dac = "dac".to_string();
    let fft = "fft".to_string();
    let out = "out".to_string();

    // We're going to do a searh where when we see a node we can record all of the paths from that node and ensure that we've exhausted it to avoid treading it again.
    let mut remaining_machines = input.connections.keys().collect_vec();

    while !remaining_machines.is_empty() {
        let mut machines_complete = Vec::new();
        for &machine in remaining_machines.iter() {
            let machine_paths = paths_to_process.get_mut(machine).unwrap();
            if let Some(path) = machine_paths.pop_front() {
                if !seen_paths.insert(path.clone()) {
                    continue;
                }
                let has_dac = path.contains(&dac);
                let has_fft = path.contains(&fft);
                let machine_known_paths = known_paths.get_mut(machine).unwrap();
                if !machine_known_paths.contains_key(path.last().unwrap()) {
                    machine_known_paths.insert(
                        path.last().unwrap().to_string(),
                        TargetPathInfo {
                            paths_with_dac: if has_dac && !has_fft { 1 } else { 0 },
                            paths_with_fft: if has_fft && !has_dac { 1 } else { 0 },
                            paths_with_both: if has_dac && has_fft { 1 } else { 0 },
                            paths_with_neither: if !has_dac && !has_fft { 1 } else { 0 },
                        },
                    );
                } else {
                    let path_info = machine_known_paths.get_mut(path.last().unwrap()).unwrap();
                    if has_dac && has_fft {
                        path_info.paths_with_both += 1;
                    } else if has_dac {
                        path_info.paths_with_dac += 1;
                    } else if has_fft {
                        path_info.paths_with_fft += 1;
                    } else {
                        path_info.paths_with_neither += 1;
                    }
                }
                let _ = machine_known_paths;
                for target in input.connections[path.last().unwrap()].iter() {
                    if path.contains(target) {
                        println!("{} is a loop in {:?}", target, path);
                        println!("FOUND LOOP!!");
                        continue;
                    }

                    if !remaining_machines.contains(&target) {
                        // We know the full map of this other target and should just use it
                        let new_path_info = known_paths[target]
                            .iter()
                            .filter(|(destination, _)| *destination == "out")
                            .last();
                        if new_path_info.is_none() {
                            continue;
                        }
                        let mut new_path_info = new_path_info.unwrap().1.clone();
                        if has_dac && has_fft {
                            new_path_info = TargetPathInfo {
                                paths_with_both: new_path_info.paths_with_both
                                    + new_path_info.paths_with_neither
                                    + new_path_info.paths_with_fft
                                    + new_path_info.paths_with_dac,
                                ..Default::default()
                            };
                        } else if has_dac {
                            new_path_info.paths_with_dac += new_path_info.paths_with_neither;
                            new_path_info.paths_with_both += new_path_info.paths_with_fft;
                            new_path_info.paths_with_neither = 0;
                            new_path_info.paths_with_fft = 0;
                        } else if has_fft {
                            new_path_info.paths_with_fft += new_path_info.paths_with_neither;
                            new_path_info.paths_with_both += new_path_info.paths_with_dac;
                            new_path_info.paths_with_neither = 0;
                            new_path_info.paths_with_dac = 0;
                        }

                        let machine_known_paths = known_paths.get_mut(machine).unwrap();
                        if !machine_known_paths.contains_key(&out) {
                            machine_known_paths.insert(out.to_string(), new_path_info.clone());
                        } else {
                            let existing_info = machine_known_paths.get_mut(&out).unwrap();
                            *existing_info += new_path_info.clone();
                        }
                        continue;
                    }

                    let mut new_vec = path.clone();
                    new_vec.push(target.to_string());
                    machine_paths.push_back(new_vec);
                }
            } else {
                machines_complete.push(machine);
            }
        }
        for mahine_to_remove in machines_complete {
            remaining_machines.remove(
                remaining_machines
                    .iter()
                    .position(|m| **m == *mahine_to_remove)
                    .unwrap(),
            );

            // Clean up some memory since we only need the paths to out
            let paths = known_paths.get_mut(mahine_to_remove).unwrap();
            let targets_to_remove = paths.keys().filter(|k| *k != "out").cloned().collect_vec();
            for target in targets_to_remove {
                paths.remove(&target);
            }
            println!(
                "{} complete, {} remaining",
                mahine_to_remove,
                remaining_machines.len()
            );
        }
    }

    known_paths["svr"]["out"].paths_with_both
}

fn parse(file: &str) -> Input {
    let file = File::open(file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let mut connections = HashMap::new();

    lines.iter().for_each(|line| {
        let (name, targets) = line.split_once(":").unwrap();
        connections.insert(
            name.trim().to_string(),
            targets
                .trim()
                .split_ascii_whitespace()
                .map(|t| t.to_string())
                .collect_vec(),
        );
    });
    Input { connections }

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

        assert_eq!(result1, 5);
    }

    #[test]
    fn test_part2() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test2.txt"));
        let result2 = part2(&input);

        assert_eq!(result2, 2);
    }
}
