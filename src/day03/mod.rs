use std::collections::{HashMap, HashSet};

pub fn part1(path: &str) -> i32 {
    let input = std::fs::read_to_string(path).unwrap();
    let lines = input.split_whitespace().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut sum = 0;
    let mut row: usize = 0;
    while row < lines.len() {
        let mut col: usize = 0;
        let mut number = "".to_string();
        let mut ok = false;
        while col < lines[row].len() {
            let current = &lines[row][col];
            if current.is_numeric() {
                number.push(*current);
            } else {
                if ok {
                    let parsed = number.parse().unwrap_or(0);
                    if parsed != 0 {
                        sum += number.parse().unwrap_or(0);
                    }
                }
                ok = false;
                number.clear();
                col += 1;
                continue;
            }

            for y in row.saturating_sub(1)..=row.saturating_add(1).min(lines.len() - 1) {
                for x in col.saturating_sub(1)..=col.saturating_add(1).min(lines[0].len() - 1) {
                    let neighbour = &lines[y][x];
                    if !neighbour.is_numeric() && neighbour != &'.' {
                        ok = true;
                        break;
                    }
                }
                if ok {
                    break;
                }
            }
            col += 1;

            if col == lines[0].len() {
                if ok {
                    let parsed = number.parse().unwrap_or(0);
                    if parsed != 0 {
                        sum += number.parse().unwrap_or(0);
                    }
                }
                ok = false;
                number.clear();
            }
        }
        row += 1;
    }

    sum
}

pub fn part2(path: &str) -> i32 {
    let input = std::fs::read_to_string(path).unwrap();
    let lines = input.split_whitespace().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut row: usize = 0;
    let mut asterisks: HashMap<usize, HashSet<i32>> = HashMap::new();
    while row < lines.len() {
        let mut col: usize = 0;
        let mut number = "".to_string();
        let mut asterisks_nearby: Vec<usize> = vec![];
        let mut ok = false;
        while col < lines[row].len() {
            let current = &lines[row][col];
            if current.is_numeric() {
                number.push(*current);
            } else {
                if ok {
                    let parsed = number.parse().unwrap_or(0);
                    for ele in &asterisks_nearby {
                        if asterisks.contains_key(&ele) {
                            asterisks.get_mut(&ele).unwrap().insert(parsed);
                        } else {
                            asterisks.insert(*ele, HashSet::from([parsed]));
                        }
                    }
                }
                ok = false;
                number.clear();
                asterisks_nearby.clear();
                col += 1;
                continue;
            }

            for y in row.saturating_sub(1)..=row.saturating_add(1).min(lines.len() - 1) {
                for x in col.saturating_sub(1)..=col.saturating_add(1).min(lines[0].len() - 1) {
                    let neighbour = &lines[y][x];
                    if !neighbour.is_numeric() && neighbour != &'.' {
                        ok = true;
                        if neighbour == &'*' {
                            asterisks_nearby.push(y * lines[0].len() + x);
                        }
                    }
                }
            }
            col += 1;

            if col == lines[0].len() {
                if ok {
                    let parsed = number.parse().unwrap_or(0);
                    for ele in &asterisks_nearby {
                        if asterisks.contains_key(&ele) {
                            asterisks.get_mut(&ele).unwrap().insert(parsed);
                        } else {
                            asterisks.insert(*ele, HashSet::from([parsed]));
                        }
                    }
                }
                ok = false;
                number.clear();
            }
        }
        row += 1;
    }

    asterisks.iter().fold(0, |acc, (_, adjacent)| {
        acc + if adjacent.len() == 2 { adjacent.iter().product() } else { 0 }
    })
}
