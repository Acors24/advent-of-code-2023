use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn a(incoming: &Direction, cell: &char) -> Vec<Direction> {
    match cell {
        '.' => vec![incoming.clone()],
        '/' => {
            match incoming {
                Direction::Up => vec![Direction::Right],
                Direction::Down => vec![Direction::Left],
                Direction::Left => vec![Direction::Down],
                Direction::Right => vec![Direction::Up],
            }
        },
        '\\' => {
            match incoming {
                Direction::Up => vec![Direction::Left],
                Direction::Down => vec![Direction::Right],
                Direction::Left => vec![Direction::Up],
                Direction::Right => vec![Direction::Down],
            }
        },
        '|' => {
            match incoming {
                Direction::Up | Direction::Down => vec![incoming.clone()],
                Direction::Left | Direction::Right => vec![Direction::Up, Direction::Down],
            }
        },
        '-' => {
            match incoming {
                Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
                Direction::Left | Direction::Right => vec![incoming.clone()],
            }
        },
        _ => panic!()
    }
}

fn next_pos(pos: &(usize, usize), direction: &Direction) -> (Option<usize>, Option<usize>) {
    match direction {
        Direction::Up => (Some(pos.0), pos.1.checked_sub(1)),
        Direction::Down => (Some(pos.0), pos.1.checked_add(1)),
        Direction::Left => (pos.0.checked_sub(1), Some(pos.1)),
        Direction::Right => (pos.0.checked_add(1), Some(pos.1)),
    }
}

fn trace(pos: &(usize, usize), grid: &Vec<Vec<char>>, direction: &Direction, energized: &mut HashSet<(usize, usize)>, visited: &mut HashSet<(usize, usize, Direction)>) {
    if visited.contains(&(pos.0, pos.1, direction.clone())) {
        return;
    } else {
        visited.insert((pos.0, pos.1, direction.clone()));
    }

    let mut pos = pos.to_owned();
    loop {
        if let (Some(x), Some(y)) = next_pos(&pos, direction) {
            if x >= grid[0].len() || y >= grid.len() {
                return;
            }

            pos = (x, y);
        } else {
            return;
        }

        energized.insert(pos);

        if grid[pos.1][pos.0] != '.' {
            break;
        }
    }
    
    let directions = a(direction, &grid[pos.1][pos.0]);
    for d in directions {
        trace(&pos, grid, &d, energized, visited);
    }
}

pub fn part1(path: &str) -> i32 {
    let input = std::fs::read_to_string(path).unwrap();
    let grid = input.split_ascii_whitespace().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut energized = HashSet::from([(0usize, 0usize)]);
    for direction in a(&Direction::Right, &grid[0][0]) {
        trace(&(0, 0), &grid, &direction, &mut energized, &mut HashSet::new());
    }

    energized.len() as i32
}

pub fn part2(path: &str) -> i32 {
    let input = std::fs::read_to_string(path).unwrap();
    let grid = input.split_ascii_whitespace().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let width = grid[0].len();
    let height = grid.len();

    let mut max = 0usize;
    for x in 0..width {
        let mut energized = HashSet::from([(x, 0usize)]);
        for direction in a(&Direction::Down, &grid[x][0]) {
            trace(&(x, 0), &grid, &direction, &mut energized, &mut HashSet::new());
        }

        max = max.max(energized.len());

        energized = HashSet::from([(x, height - 1)]);
        for direction in a(&Direction::Up, &grid[x][height - 1]) {
            trace(&(x, height - 1), &grid, &direction, &mut energized, &mut HashSet::new());
        }
    
        max = max.max(energized.len());
    }

    for y in 0..height {
        let mut energized = HashSet::from([(0, y)]);
        for direction in a(&Direction::Right, &grid[0][y]) {
            trace(&(0, y), &grid, &direction, &mut energized, &mut HashSet::new());
        }

        max = max.max(energized.len());

        energized = HashSet::from([(width - 1, y)]);
        for direction in a(&Direction::Left, &grid[width - 1][y]) {
            trace(&(width - 1, y), &grid, &direction, &mut energized, &mut HashSet::new());
        }

        max = max.max(energized.len());
    }
    
    max as i32
}
