use std::collections::HashSet;

fn walk(
    pos: (usize, usize),
    grid: &Vec<Vec<char>>,
    steps: i32
) -> i32 {
    if steps <= 0 {
        return 1;
    }

    let width = grid[0].len();
    let height = grid.len();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut positions = 0;
    let mut queue = Vec::from([(pos, steps)]);

    while let Some(((x, y), steps)) = queue.pop() {
        if grid[y][x] == '#' || visited.contains(&(x, y)) {
            continue;
        }

        visited.insert((x, y));

        if steps % 2 == 0 {
            positions += 1;
        }

        if steps <= 0 {
            continue;
        }


        for dir in 0..4 {
            match dir {
                0 if y > 0 && !visited.contains(&(x, y - 1)) => {
                    queue.insert(0, ((x, y - 1), steps - 1));
                }
                1 if x < width - 1 && !visited.contains(&(x + 1, y)) => {
                    queue.insert(0, ((x + 1, y), steps - 1));
                }
                2 if y < height - 1 && !visited.contains(&(x, y + 1)) => {
                    queue.insert(0, ((x, y + 1), steps - 1));
                }
                3 if x > 0 && !visited.contains(&(x - 1, y)) => {
                    queue.insert(0, ((x - 1, y), steps - 1));
                }
                _ => continue,
            }
        }
    }

    positions
}

pub fn part1(path: &str) -> i32 {
    let input = std::fs::read_to_string(path).unwrap();
    let grid = input
        .split_whitespace()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut pos = (0, 0);
    'outer: for (y, row) in grid.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            if ch == &'S' {
                pos = (x, y);
                break 'outer;
            }
        }
    }

    let steps = 64;
    walk(pos, &grid, steps)
}

pub fn part2(path: &str) -> i32 {
    let input = std::fs::read_to_string(path).unwrap();
    let grid = input
        .split_whitespace()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut pos = (0, 0);
    'outer: for (y, row) in grid.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            if ch == &'S' {
                pos = (x, y);
                break 'outer;
            }
        }
    }

    let steps = 201;
    walk(pos, &grid, steps)
}
