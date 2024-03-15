use std::{collections::HashMap, fs};

fn extract_and_sum(v: Vec<String>) -> u32 {
    v.iter()
        .map(|s| {
            let v = s
                .chars()
                .filter_map(|ch| ch.to_digit(10))
                .collect::<Vec<_>>();
            v.first().unwrap() * 10 + v.last().unwrap()
        })
        .sum::<u32>()
}

pub fn part1(path: &str) -> u32 {
    extract_and_sum(
        fs::read_to_string(path)
            .unwrap()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<_>>(),
    )
}

pub fn part2(path: &str) -> u32 {
    let map: HashMap<&str, u32> = [
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .iter()
    .cloned()
    .collect();

    let input = fs::read_to_string(path).unwrap();

    let lines: Vec<_> = input
        .split_whitespace()
        .map(|line: &str| {
            let mut new_line = line.to_string();

            let mut first: usize = line.len();
            let mut found_digit = "";
            for (s, _) in &map {
                match new_line.find(s) {
                    Some(i) => {
                        if i < first {
                            first = i;
                            found_digit = s;
                        }
                    }
                    None => (),
                }
            }

            let first_digit = new_line.find(|ch: char| ch.is_numeric()).unwrap();
            if first < first_digit {
                new_line.replace_range(
                    first..first + found_digit.len(),
                    &map.get(found_digit).unwrap().to_string(),
                );
            }

            let mut last: usize = 0;
            let mut found_digit = "";
            for (s, _) in &map {
                match new_line.find(s) {
                    Some(i) => {
                        if i > last {
                            last = i;
                            found_digit = s;
                        }
                    }
                    None => (),
                }
            }

            let last_digit = new_line.rfind(|ch: char| ch.is_numeric()).unwrap();
            if last > last_digit {
                new_line.replace_range(
                    last..last + found_digit.len(),
                    &map.get(found_digit).unwrap().to_string(),
                );
            }

            new_line
        })
        .collect();

    extract_and_sum(lines)
}
