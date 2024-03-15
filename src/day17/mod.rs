#[derive(Debug, Clone)]
struct Cell {
    x: usize,
    y: usize,
    visited: bool,
    distance: i32,
    direction: i8,
    dir_count: i8,
}

impl Cell {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            visited: false,
            distance: i32::MAX,
            direction: 1,
            dir_count: 0,
        }
    }
}

fn walk(grid: &Vec<Vec<i32>>) -> i32 {
    let width = grid[0].len();
    let height = grid.len();

    let mut cells = vec![vec![Cell::new(); width]; height];
    for y in 0..height {
        for x in 0..width {
            cells[y][x].x = x;
            cells[y][x].y = y;
        }
    }

    cells[0][0].distance = 0;
    cells[0][0].direction = 1;
    cells[0][0].dir_count = 0;

    for _ in 0..(width * height) {
        let clone = cells.clone();
        let current = clone
            .iter()
            .filter_map(|row| row.iter().filter(|c| !c.visited).min_by_key(|c| c.distance))
            .min_by_key(|c| c.distance)
            .unwrap();

        let available_directions = (0..4)
            .filter(|&d| d != (current.direction + 2i8).rem_euclid(4))
            .collect::<Vec<_>>();

        for d in &available_directions {
            let neighbor_pos = match d {
                0 if current.y > 0 => (current.x, current.y - 1),
                1 if current.x < width - 1 => (current.x + 1, current.y),
                2 if current.y < height - 1 => (current.x, current.y + 1),
                3 if current.x > 0 => (current.x - 1, current.y),
                _ => continue,
            };
            let c_dir_count = if d == &current.direction {
                current.dir_count + 1
            } else {
                1
            };
            if c_dir_count > 3 {
                continue;
            }
            let neighbor = &mut cells[neighbor_pos.1][neighbor_pos.0];
            if neighbor.visited {
                continue;
            }
            let new_n_dist = current.distance + grid[neighbor.y][neighbor.x];
            if new_n_dist < neighbor.distance {
                neighbor.distance = new_n_dist;
                neighbor.direction = *d;
                neighbor.dir_count = c_dir_count;
            }
        }

        cells[current.y][current.x].visited = true;
        if cells[height - 1][width - 1].visited {
            break;
        }
    }

    // for row in &cells {
    //     for cell in row {
    //         print!(
    //             "{:} ",
    //             match cell.direction {
    //                 0 => '↑',
    //                 1 => '→',
    //                 2 => '↓',
    //                 3 => '←',
    //                 _ => panic!(),
    //             }
    //         );
    //     }
    //     println!()
    // }

    cells[height - 1][width - 1].distance
}

pub fn part1(path: &str) -> i32 {
    let input = std::fs::read_to_string(path).unwrap();
    let grid = input
        .split_ascii_whitespace()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    walk(&grid)
}
