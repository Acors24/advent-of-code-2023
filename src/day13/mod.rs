fn to_number(line: &Vec<char>) -> usize {
    line.iter()
        .enumerate()
        .map(|(i, c)| if c == &'#' { 1 } else { 0 } * 2usize.pow(i as u32))
        .sum::<usize>()
}

fn compare(v1: &Vec<usize>, v2: &Vec<usize>) -> bool {
    v1.iter().zip(v2.iter()).all(|(a, b)| a == b)
}

fn compare_fixing(v1: &Vec<usize>, v2: &Vec<usize>) -> bool {
    let mut fixed = false;
    for (a, b) in v1.iter().zip(v2.iter()) {
        if a != b {
            if !fixed && (a ^ b).is_power_of_two() {
                fixed = true;
            } else {
                return false;
            }
        }
    }

    fixed
}

fn find_reflection(nums: &Vec<usize>, compare_fn: fn(&Vec<usize>, &Vec<usize>) -> bool) -> usize {
    (1..nums.len())
        .filter(|&i| {
            compare_fn(
                &nums[..i].to_owned().into_iter().rev().collect::<Vec<_>>(),
                &nums[i..].to_vec(),
            )
        })
        .max()
        .unwrap_or_default()
}

fn find(pattern: &Vec<&str>, compare_fn: fn(&Vec<usize>, &Vec<usize>) -> bool) -> usize {
    let rows = pattern
        .iter()
        .map(|row| to_number(&row.chars().collect::<Vec<_>>()))
        .collect::<Vec<_>>();

    let cols = (0..pattern[0].len())
        .map(|i| {
            to_number(
                &pattern
                    .iter()
                    .map(|row| row.chars().nth(i).unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let (r, c) = (
        find_reflection(&rows, compare_fn),
        find_reflection(&cols, compare_fn)
    );

    if r > c {
        100 * r
    } else {
        c
    }
}

fn solve(path: &str, compare_fn: fn(&Vec<usize>, &Vec<usize>) -> bool) -> u64 {
    let input = std::fs::read_to_string(path).unwrap();
    let patterns = input
        .split("\n\n")
        .map(|pattern| pattern.split_ascii_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    patterns
        .iter()
        .map(|p| find(p, compare_fn) as u64)
        .sum::<u64>()
}

pub fn part1(path: &str) -> u64 {
    solve(path, compare)
}

pub fn part2(path: &str) -> u64 {
    solve(path, compare_fixing)
}
