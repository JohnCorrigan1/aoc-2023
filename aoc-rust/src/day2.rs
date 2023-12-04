use crate::helpers::get_lines;

const MAX_RED: u16 = 12;
const MAX_GREEN: u16 = 13;
const MAX_BLUE: u16 = 14;

struct Round {
    red: u16,
    green: u16,
    blue: u16,
}

impl Round {
    fn get_round(round: &str) -> Round {
        let round: Vec<&str> = round.split(", ").collect();
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for color in round {
            let group: Vec<&str> = color.split(" ").collect();
            let color = group[1];
            let value = group[0].parse::<u16>().unwrap();
            match color {
                "red" => red = value,
                "green" => green = value,
                "blue" => blue = value,
                _ => panic!("Invalid color"),
            }
        }

        Round { red, green, blue }
    }
}

pub fn run() {
    let input = get_lines("day2");

    println!("Day 2");
    println!("-------------------------------");
    println!("Part 1: {}", q1(&input));
    println!("Part 2: {}", q2(&input));
    println!();
}

fn q1(input: &Vec<String>) -> u16 {
    let mut total = 0;
    for line in input.clone() {
        let id = get_id(&line);
        let rounds = get_rounds(&line);
        if is_valid(&rounds) {
            total += id as u16;
        }
    }
    total
}

fn q2(input: &Vec<String>) -> u16 {
    let mut total = 0;
    for line in input.clone() {
        let rounds = get_rounds(&line);
        total += get_powers(rounds);
    }
    total
}

fn get_id(line: &str) -> u16 {
    let line: Vec<&str> = line.split(" ").collect();
    let mut line: Vec<&str> = line[1].split("").collect();
    line.pop();
    line.pop();
    let id = line.join("").parse::<u16>().unwrap();
    id
}

fn get_rounds(line: &str) -> Vec<Round> {
    let line: Vec<&str> = line.split(": ").collect();
    let rounds: Vec<&str> = line[1].split("; ").collect();
    rounds
        .iter()
        .map(|round| Round::get_round(round))
        .collect::<Vec<Round>>()
}

fn is_valid(rounds: &Vec<Round>) -> bool {
    for round in rounds {
        if round.red > MAX_RED || round.green > MAX_GREEN || round.blue > MAX_BLUE {
            return false;
        }
    }
    true
}

fn get_powers(rounds: Vec<Round>) -> u16 {
    let mut green: u16 = 0;
    let mut red: u16 = 0;
    let mut blue: u16 = 0;

    for round in rounds {
        if round.red > red {
            red = round.red;
        }
        if round.green > green {
            green = round.green;
        }
        if round.blue > blue {
            blue = round.blue;
        }
    }
    red * green * blue
}
