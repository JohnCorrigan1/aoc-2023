use crate::helpers::get_lines;
use std::collections::HashMap;

pub fn run() {
    let input = get_lines("day4");

    println!("Day 4");
    println!("-------------------------------");
    println!("Part 1: {}", q1(&input));
    println!("Part 2: {}", q2(&input));
    println!();
}

fn q1(input: &Vec<String>) -> u32 {
    let mut total = 0;

    for line in input {
        let count = get_winners(line);
        if count == 1 {
            total += 1;
        }
        if count > 1 {
            total += 2_u32.pow(count - 1);
        }
    }
    total
}

fn q2(input: &Vec<String>) -> u32 {
    let mut cards_won: HashMap<u32, u32> = HashMap::new();
    let mut current_card = 0;
    for line in input {
        current_card += 1;
        cards_won
            .entry(current_card)
            .and_modify(|e| *e += 1)
            .or_insert(1);
        let count = get_winners(line);
        for _i in 0..cards_won.get(&current_card).unwrap().clone() {
            for j in current_card + 1..current_card + 1 + count {
                cards_won.entry(j).and_modify(|e| *e += 1).or_insert(1);
            }
        }
    }

    cards_won.iter().map(|(_k, v)| v).sum::<u32>()
}

fn get_winners(line: &str) -> u32 {
    let line = line.split(": ").collect::<Vec<&str>>()[1];
    let line = line.split(" | ").collect::<Vec<&str>>();
    let winners = line[0].split_whitespace().collect::<Vec<&str>>();
    let losers = line[1].split_whitespace().collect::<Vec<&str>>();
    let mut count: u32 = 0;
    for winner in winners {
        if losers.contains(&winner) {
            count += 1;
        }
    }
    count
}
