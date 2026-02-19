use std::fs;

use day1::{part_one, part_two};

fn main() {
    let input = read_input();
    let part1 = part_one(&input);
    let part2 = part_two(&input);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn read_input() -> String {
    let contents =
        fs::read_to_string("input.txt").expect("Should have been able to read input.txt");
    return contents;
}
