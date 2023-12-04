use crate::helpers::get_lines;
use std::collections::HashMap;

enum PartType {
    Gear,
    Part,
}

impl PartType {
    fn is_of_type(&self, character: char) -> bool {
        match self {
            PartType::Gear => character == '*',
            PartType::Part => !character.is_digit(10) && character != '.',
        }
    }
}

pub fn run() {
    let input = get_lines("day3");
    let grid: Vec<Vec<char>> = input.iter().map(|x| x.chars().collect()).collect();
    let height = grid.len();
    let width = grid[0].len();
    let gear = PartType::Gear;
    let part = PartType::Part;

    println!("Day 3");
    println!("-------------------------------");
    println!("Part 1: {}", map_grid(&grid, height, width, part));
    println!("Part 2: {}", map_grid(&grid, height, width, gear));
    println!();
}

fn map_grid(grid: &Vec<Vec<char>>, height: usize, width: usize, part_type: PartType) -> u32 {
    let mut gear_map: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    let mut total = 0;
    for row in 0..height {
        let mut skipto = 0;
        for col in 0..width - 1 {
            if col >= skipto {
                skipto = 0;
                let current = grid[row][col];
                if current.is_digit(10) {
                    let part_number = get_part_number(grid, row, col.clone());
                    let end = col + part_number.to_string().len();
                    for column in col..end {
                        if let Some((row, col)) =
                            check_part(grid, row as i32, column as i32, &part_type)
                        {
                            match part_type {
                                PartType::Gear => {
                                    gear_map
                                        .entry((row, col))
                                        .and_modify(|nums| nums.push(part_number))
                                        .or_insert(vec![part_number]);
                                }
                                PartType::Part => {
                                    total += part_number;
                                }
                            }
                            break;
                        }
                    }
                    skipto = end;
                }
            }
        }
    }
    match part_type {
        PartType::Gear => {
            for gear in gear_map {
                if gear.1.len() == 2 {
                    total += gear.1[0] * gear.1[1]
                }
            }
            total
        }
        _ => total,
    }
}

fn check_part(
    grid: &Vec<Vec<char>>,
    row: i32,
    col: i32,
    part_type: &PartType,
) -> Option<(usize, usize)> {
    let arround = vec![
        (row - 1, col),
        (row + 1, col),
        (row, col - 1),
        (row, col + 1),
        (row - 1, col - 1),
        (row - 1, col + 1),
        (row + 1, col - 1),
        (row + 1, col + 1),
    ];

    for (row, col) in arround {
        if row < grid.len() as i32 && col < grid[0].len() as i32 && row >= 0 && col >= 0 {
            if part_type.is_of_type(grid[row as usize][col as usize]) {
                return Some((row as usize, col as usize));
            }
        }
    }

    None
}

fn get_part_number(grid: &Vec<Vec<char>>, row: usize, mut col: usize) -> u32 {
    let mut nums: Vec<String> = vec![];
    while grid[row][col].is_digit(10) && col != grid[0].len() {
        nums.push(grid[row][col].to_string());
        if col == grid[0].len() - 1 {
            break;
        }
        col += 1;
    }

    let part_number = nums.join("");
    part_number.parse().unwrap()
}
