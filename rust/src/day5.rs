use crate::helpers::get_lines;
//use num::bigint::BigInt;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn run() {
    let input = get_lines("daytest");

    let seeds: Vec<u64> = input.clone()[0].split(": ").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    println!("Day 5");
    println!("-------------------------------");
    println!("Part 1: {}", q1(&input, &seeds));
    println!("Part 2: {}", q2(&input, &seeds));
    println!();
}

fn q1(input: &Vec<String>, seeds: &Vec<u64>) -> u64 {
    let mut location: HashMap<u64, Vec<u64>> = seeds.iter().map(|&x| (x, vec![x])).collect();
    let maps = input.split(|x| x.is_empty()).collect::<Vec<&[String]>>();
    for i in 1..maps.len() {
        for seed in seeds {
            let current_seed = &location.get(&seed).unwrap().clone();
            let current_seed = &current_seed[current_seed.len() - 1];
            for j in 1..maps[i].len() {
                let map = &maps[i][j]
                    .split_whitespace()
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
                let max = &map[1] + &map[2];
                let min = &map[1];
                if current_seed >= &min && current_seed < &max {
                    location
                        .get_mut(&seed)
                        .unwrap()
                        .push(current_seed.abs_diff(map[1]) + map[0]);
                    break;
                }
            }
            if &location.get(&seed).unwrap().len() == &i {
                location.get_mut(&seed).unwrap().push(*current_seed);
            }
        }
    }
    println!("made it here!!!!!!!!! 'yup");
    let mut smallest = 1_000_000_000_000_000_000;
    for loc in location.iter() {
        if loc.1[loc.1.len() - 1] < smallest {
            smallest = loc.1[loc.1.len() - 1];
        }
    }
    smallest
}

fn q2(input: &Vec<String>, seeds: &Vec<u64>) -> u64 {
    //every other seed in seeds
    let mut seeds_hash: HashSet<u64> = HashSet::new();
    let mut new_seeds: Vec<u64> = vec![];
    for i in 0..seeds.len() {
        if i % 2 == 0 {
            for j in seeds[i]..seeds[i] + seeds[i + 1] {
                if let Some(_) = seeds_hash.get(&j) {
                    println!("skipping {}", j);
                    continue;
                } else {
                    println!("ok wtf: {}", new_seeds.len());
                    new_seeds.push(j);
                    seeds_hash.insert(j);
                }
            }
        }
    }
    //println!("{:?}", seeds);
    //println!();
    //println!("{:?}", new_seeds);
    q1(input, &new_seeds)
}

fn get_ranges(input: &Vec<String>) -> Vec<(usize, u64, u64)> {
    let mut ranges: Vec<(usize, u64, u64)> = vec![];
    let maps = input.split(|x| x.is_empty()).collect::<Vec<&[String]>>();
    for i in 1..maps.len() {
        for j in 1..maps[i].len() {
            let map = &maps[i][j]
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            ranges.push((i, map[1], map[1] + map[2]));
        }
    }

    ranges
}

fn real(input: &Vec<String>, seeds: &Vec<u64>) -> u64 {
    let mut location: HashMap<u64, Vec<u64>> = seeds.iter().map(|&x| (x, vec![x])).collect();
    let maps = input.split(|x| x.is_empty()).collect::<Vec<&[String]>>();
    for i in 1..maps.len() {
        for seed in seeds {
            let current_seed = &location.get(&seed).unwrap().clone();
            let current_seed = &current_seed[current_seed.len() - 1];
            for j in 1..maps[i].len() {
                let map = &maps[i][j]
                    .split_whitespace()
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
                let max = &map[1] + &map[2];
                let min = &map[1];
                if current_seed >= &min && current_seed < &max {
                    location
                        .get_mut(&seed)
                        .unwrap()
                        .push(current_seed.abs_diff(map[1]) + map[0]);
                    break;
                }
            }
            if &location.get(&seed).unwrap().len() == &i {
                location.get_mut(&seed).unwrap().push(*current_seed);
            }
        }
    }
    println!("made it here!!!!!!!!! 'yup");
    let mut smallest = 1_000_000_000_000_000_000;
    for loc in location.iter() {
        if loc.1[loc.1.len() - 1] < smallest {
            smallest = loc.1[loc.1.len() - 1];
        }
    }
    smallest
}
