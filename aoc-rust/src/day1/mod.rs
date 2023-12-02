use itertools::Itertools;

use crate::helpers::get_lines;
struct WordNum {
    num_str: String,
    index: String,
}

struct NumPosition {
    num: String,
    index: u32,
}

pub fn run() {
    let input = get_lines("day1");

    println!("Day 1");
    println!("-------------------------------");
    println!("Part 1: {}", q1(&input));
    println!("Part 2: {}", q2(&input));
}

fn q1(input: &Vec<String>) -> u32 {
    let mut total = 0;
    for line in input.clone() {
        total += get_nums(line);
    }
    total
}

fn q2(input: &Vec<String>) -> u32 {
    let mut total = 0;
    for line in input.clone() {
        let word_nums = get_word_nums(line.clone());
        let char_nums = get_char_nums(line.clone());
        let all_nums: Vec<&NumPosition> = word_nums
            .iter()
            .chain(char_nums.iter())
            .map(|x| x.clone())
            .sorted_by(|a, b| a.index.cmp(&b.index).clone())
            .collect();

        total += (all_nums[0].num.clone() + &all_nums[all_nums.len() - 1].num.clone())
            .parse::<u32>()
            .unwrap();
    }
    total
}

fn get_char_nums(line: String) -> Vec<NumPosition> {
    let mut nums: Vec<NumPosition> = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    for (i, c) in chars.iter().enumerate() {
        match c.is_digit(10) {
            true => nums.push(NumPosition {
                num: c.to_string(),
                index: i as u32,
            }),
            _ => (),
        }
    }
    nums
}

fn get_nums(line: String) -> u32 {
    let chars: Vec<char> = line.chars().collect();
    let nums = chars
        .iter()
        .filter(|c| c.is_digit(10))
        .collect::<Vec<&char>>();

    let num: String = String::from(nums[0].clone()) + &String::from(nums[nums.len() - 1].clone());
    num.parse::<u32>().unwrap()
}

fn get_word_nums(line: String) -> Vec<NumPosition> {
    let real: Vec<WordNum> = vec![
        WordNum {
            num_str: String::from("one"),
            index: "1".to_string(),
        },
        WordNum {
            num_str: String::from("two"),
            index: "2".to_string(),
        },
        WordNum {
            num_str: String::from("three"),
            index: "3".to_string(),
        },
        WordNum {
            num_str: String::from("four"),
            index: "4".to_string(),
        },
        WordNum {
            num_str: String::from("five"),
            index: "5".to_string(),
        },
        WordNum {
            num_str: String::from("six"),
            index: "6".to_string(),
        },
        WordNum {
            num_str: String::from("seven"),
            index: "7".to_string(),
        },
        WordNum {
            num_str: String::from("eight"),
            index: "8".to_string(),
        },
        WordNum {
            num_str: String::from("nine"),
            index: "9".to_string(),
        },
    ];
    let line = line.clone();
    let mut word_nums: Vec<NumPosition> = Vec::new();
    for word in real {
        match line.contains(&word.num_str) {
            true => {
                for (i, _) in line.match_indices(&word.num_str) {
                    word_nums.push(NumPosition {
                        num: word.index.clone(),
                        index: i as u32,
                    })
                }
            }
            _ => (),
        }
    }
    word_nums
}
