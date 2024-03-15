use std::collections::HashMap;

fn fit(group: &str, lengths: &[i64], mem: &mut HashMap<(usize, usize, usize), i64>) -> i64 {
    let len = if let Some(len) = lengths.iter().nth(0) {
        *len as usize
    } else {
        if group.contains('#') {
            return 0;
        } else {
            return 1;
        }
    };

    if group.len() < len {
        return 0;
    }

    let mut sum = 0;
    let mut i = 0;

    while i <= group.len() - len {
        if group[..i].contains('#') {
            return sum;
        }
        let s = &group[i..i + len];
        if !s.contains('.') {
            if i > 0 {
                if let Some('#') = group.chars().nth(i - 1) {
                    i += 1;
                    continue;
                }
            }
            if let Some('#') = group.chars().nth(i + len) {
                i += 1;
                continue;
            } else {
                if &lengths[1..].len() == &0 && !group[i + len..].contains('#') {
                    sum += 1;
                } else if let Some(next) = group.get(i + len + 1..) {
                    let data = (group.len(), i, lengths.len());
                    if let Some(result) = mem.get(&data) {
                        sum += result;
                    } else {
                        let result = fit(next, &lengths[1..], mem);
                        mem.insert(data, result);
                        sum += result;
                    };
                } else {
                    return sum;
                }
            }
        }
        i += 1;
    }

    sum
}

pub fn part1(path: &str) -> i64 {
    let input = std::fs::read_to_string(path).unwrap();
    let lines = input
        .split('\n')
        .map(|line| line.split_once(' ').unwrap())
        .map(|(line, pattern)| {
            (
                line,
                pattern
                    .split(',')
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    lines
        .iter()
        .map(|(s, pat)| fit(s, pat, &mut HashMap::new()))
        .sum()
}

pub fn part2(path: &str) -> i64 {
    let input = std::fs::read_to_string(path).unwrap();
    let lines = input
        .split('\n')
        .map(|line| line.split_once(' ').unwrap())
        .map(|(line, pattern)| {
            let new_line = Vec::from([line]).repeat(5).join("?");
            let pattern = pattern
                .split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            let new_pattern = pattern.repeat(5);
            (new_line, new_pattern)
        })
        .collect::<Vec<_>>();

    lines
        .iter()
        .map(|(s, pat)| fit(s, pat, &mut HashMap::new()))
        .sum()
}
