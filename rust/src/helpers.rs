use std::fs;

pub fn get_input(day: &str) -> String {
    let filename = format!("../inputs/{}.txt", day);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    return contents;
}

pub fn get_lines(day: &str) -> Vec<String> {
    let input = get_input(day);
    input.lines().map(|s| s.to_string()).collect()
}
