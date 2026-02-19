use std::collections::HashMap;

pub fn part_one(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut tens = 0;
        let mut ones = 0;
        for char in line.chars() {
            if char.is_numeric() {
                if tens == 0 {
                    tens = char.to_digit(10).expect("Failed to get digit")
                }
                ones = char.to_digit(10).expect("Failed to get digit")
            }
        }
        sum += tens * 10 + ones;
    }

    return sum;
}

pub fn part_two(input: &str) -> u32 {
    let mut sum = 0;
    let word_number_map: HashMap<&str, &str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    for line in input.lines() {
        let mut digits = Vec::new();
        for i in 0..line.len() {
            let part = &line[i..];
            if let Some(c) = part.chars().next() {
                if c.is_ascii_digit() {
                    digits.push(c);
                }
            }
            for (word, digit) in &word_number_map {
                if part.starts_with(word) {
                    let number: Vec<char> = digit.chars().collect();
                    digits.push(number[0]);
                }
            }
        }
        if let (Some(first), Some(last)) = (digits.first(), digits.last()) {
            let line_sum = format!("{first}{last}").parse::<u32>().unwrap();
            sum += line_sum;
        }
    }

    return sum;
}
