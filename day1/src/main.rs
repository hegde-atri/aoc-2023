use std::fs;

use day1::{eleven_method, part_two};

fn main() {
    let input = read_input();
    let eleven_sum = part_two(&input);
    println!("{eleven_sum}");
}

fn read_input() -> String {
    let contents =
        fs::read_to_string("input.txt").expect("Should have been able to read input.txt");
    return contents;
}
