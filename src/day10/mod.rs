use std::collections::VecDeque;

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

struct Walker {
    pos: Pos,
    previous_pos: Pos,
    steps: i32,
    angle: i32,
    direction: Direction,
    rotating: bool,
}

impl Walker {
    fn new(x: usize, y: usize) -> Self {
        Walker {
            pos: Pos::new(x, y),
            previous_pos: Pos::new(x, y),
            steps: 0,
            angle: 0,
            direction: Direction::None,
            rotating: true,
        }
    }

    fn move_to(&mut self, new_pos: &Pos) {
        let new_vec = (
            new_pos.x as i32 - self.pos.x as i32,
            new_pos.y as i32 - self.pos.y as i32,
        );
        let new_direction = match new_vec {
            (1, 0) => Direction::East,
            (-1, 0) => Direction::West,
            (0, -1) => Direction::North,
            (0, 1) => Direction::South,
            _ => panic!(),
        };
        if self.direction != Direction::None && self.rotating {
            match self.direction {
                Direction::North => match new_direction {
                    Direction::North => (),
                    Direction::East => {
                        self.angle += 1;
                    }
                    Direction::West => {
                        self.angle -= 1;
                    }
                    _ => panic!(),
                },
                Direction::East => match new_direction {
                    Direction::East => (),
                    Direction::South => {
                        self.angle += 1;
                    }
                    Direction::North => {
                        self.angle -= 1;
                    }
                    _ => panic!(),
                },
                Direction::South => match new_direction {
                    Direction::South => (),
                    Direction::West => {
                        self.angle += 1;
                    }
                    Direction::East => {
                        self.angle -= 1;
                    }
                    _ => panic!(),
                },
                Direction::West => match new_direction {
                    Direction::West => (),
                    Direction::North => {
                        self.angle += 1;
                    }
                    Direction::South => {
                        self.angle -= 1;
                    }
                    _ => panic!(),
                },
                _ => panic!(),
            }
        }
        self.previous_pos = self.pos;
        self.pos = *new_pos;
        self.steps += 1;
        self.direction = new_direction;
    }
}

#[derive(PartialEq, Debug)]
enum Direction {
    None,
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, PartialEq)]
enum CellType {
    Other,
    Pipe, // part of the loop
    Inside,
}

impl std::fmt::Display for CellType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CellType::Other => '.',
                CellType::Pipe => '#',
                CellType::Inside => 'I',
            }
        )
    }
}

fn get_connections(pipe: &char) -> Vec<Direction> {
    match pipe {
        'S' => vec![
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ],
        '|' => vec![Direction::North, Direction::South],
        '-' => vec![Direction::West, Direction::East],
        'L' => vec![Direction::North, Direction::East],
        'J' => vec![Direction::North, Direction::West],
        '7' => vec![Direction::West, Direction::South],
        'F' => vec![Direction::East, Direction::South],
        _ => panic!(),
    }
}

fn find_next(field: &Vec<Vec<(char, CellType)>>, walker: &Walker) -> Pos {
    let pos = &walker.pos;
    let current_pipe = field[walker.pos.y][walker.pos.x].0;

    {
        let new_pos = Pos::new(pos.x.saturating_sub(1), pos.y);
        if new_pos != *pos && new_pos != walker.previous_pos {
            let (x, y) = (new_pos.x, new_pos.y);
            let new_pipe = field[y][x].0;
            if get_connections(&current_pipe).contains(&Direction::West)
                && get_connections(&new_pipe).contains(&Direction::East)
            {
                return new_pos;
            }
        }
    }
    {
        let new_pos = Pos::new(pos.x.saturating_add(1).min(field[0].len() - 1), pos.y);
        if new_pos != *pos && new_pos != walker.previous_pos {
            let (x, y) = (new_pos.x, new_pos.y);
            let new_pipe = field[y][x].0;
            if get_connections(&current_pipe).contains(&Direction::East)
                && get_connections(&new_pipe).contains(&Direction::West)
            {
                return new_pos;
            }
        }
    }
    {
        let new_pos = Pos::new(pos.x, pos.y.saturating_sub(1));
        if new_pos != *pos && new_pos != walker.previous_pos {
            let (x, y) = (new_pos.x, new_pos.y);
            let new_pipe = field[y][x].0;
            if get_connections(&current_pipe).contains(&Direction::North)
                && get_connections(&new_pipe).contains(&Direction::South)
            {
                return new_pos;
            }
        }
    }
    {
        let new_pos = Pos::new(pos.x, pos.y.saturating_add(1).min(field.len() - 1));
        if new_pos != *pos && new_pos != walker.previous_pos {
            let (x, y) = (new_pos.x, new_pos.y);
            let new_pipe = field[y][x].0;
            if get_connections(&current_pipe).contains(&Direction::South)
                && get_connections(&new_pipe).contains(&Direction::North)
            {
                return new_pos;
            }
        }
    }

    panic!("Nie znaleziono kolejnego pola.")
}

pub fn part1(path: &str) -> i32 {
    let input = std::fs::read_to_string(path).unwrap();
    let field = input
        .split_whitespace()
        .map(|line| {
            line.chars()
                .map(|c| (c, CellType::Other))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let (mut start_x, mut start_y): (usize, usize) = (0, 0);
    'outer: for (y, line) in field.iter().enumerate() {
        for (x, col) in line.iter().enumerate() {
            if col.0 == 'S' {
                (start_x, start_y) = (x, y);
                break 'outer;
            }
        }
    }

    let mut walker = Walker::new(start_x, start_y);

    walker.move_to(&find_next(&field, &walker));
    while walker.pos != Pos::new(start_x, start_y) {
        walker.move_to(&find_next(&field, &walker));
    }

    walker.steps / 2
}

fn mark_along_path(
    field: &mut Vec<Vec<(char, CellType)>>,
    walker: &Walker,
    inside_queue: &mut VecDeque<Pos>,
) {
    let (width, height) = (field[0].len(), field.len());
    let (x, y) = (walker.pos.x, walker.pos.y);
    match walker.direction {
        Direction::North => {
            if walker.angle.is_negative() {
                inside_queue.push_back(Pos::new(x.saturating_sub(1), y));
                inside_queue.push_back(Pos::new(x.saturating_sub(1), (y + 1).min(height - 1)));
            } else {
                inside_queue.push_back(Pos::new(x.saturating_add(1).min(width - 1), y));
                inside_queue.push_back(Pos::new(
                    x.saturating_add(1).min(width - 1),
                    (y + 1).min(height - 1),
                ));
            }
        }
        Direction::East => {
            if walker.angle.is_negative() {
                inside_queue.push_back(Pos::new(x.saturating_sub(1), y.saturating_sub(1)));
                inside_queue.push_back(Pos::new(x, y.saturating_sub(1)));
            } else {
                inside_queue.push_back(Pos::new(
                    x.saturating_sub(1),
                    y.saturating_add(1).min(height - 1),
                ));
                inside_queue.push_back(Pos::new(x, y.saturating_add(1).min(height - 1)));
            }
        }
        Direction::South => {
            if walker.angle.is_negative() {
                inside_queue.push_back(Pos::new(
                    x.saturating_add(1).min(width - 1),
                    y.saturating_sub(1),
                ));
                inside_queue.push_back(Pos::new(x.saturating_add(1).min(width - 1), y));
            } else {
                inside_queue.push_back(Pos::new(x.saturating_sub(1), y.saturating_sub(1)));
                inside_queue.push_back(Pos::new(x.saturating_sub(1), y));
            }
        }
        Direction::West => {
            if walker.angle.is_negative() {
                inside_queue.push_back(Pos::new(x, y.saturating_add(1).min(height - 1)));
                inside_queue.push_back(Pos::new(
                    (x + 1).min(width - 1),
                    y.saturating_add(1).min(height - 1),
                ));
            } else {
                inside_queue.push_back(Pos::new(x, y.saturating_sub(1)));
                inside_queue.push_back(Pos::new((x + 1).min(width - 1), y.saturating_sub(1)));
            }
        }
        _ => panic!(),
    }
}

fn spread(field: &mut Vec<Vec<(char, CellType)>>, queue: &mut VecDeque<Pos>, cell_type: &CellType) {
    while let Some(pos) = queue.pop_front() {
        if field[pos.y][pos.x].1 == CellType::Pipe {
            continue;
        }

        field[pos.y][pos.x].1 = cell_type.clone();

        let min_x = pos.x.saturating_sub(1);
        let max_x = (pos.x + 1).min(field[0].len() - 1);
        let min_y = pos.y.saturating_sub(1);
        let max_y = (pos.y + 1).min(field.len() - 1);

        (min_y..=max_y).for_each(|y| {
            (min_x..=max_x).for_each(|x| {
                if field[y][x].1 == CellType::Other {
                    queue.push_back(Pos::new(x, y));
                }
            });
        });
    }
}

pub fn part2(path: &str) -> usize {
    let input = std::fs::read_to_string(path).unwrap();
    let mut field = input
        .split_whitespace()
        .map(|line| {
            line.chars()
                .map(|c| (c, CellType::Other))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let (mut start_x, mut start_y): (usize, usize) = (0, 0);
    'outer: for (y, line) in field.iter().enumerate() {
        for (x, col) in line.iter().enumerate() {
            if col.0 == 'S' {
                (start_x, start_y) = (x, y);
                break 'outer;
            }
        }
    }

    let mut walker = Walker::new(start_x, start_y);
    field[start_y][start_x].1 = CellType::Pipe;

    walker.move_to(&find_next(&field, &walker));
    field[walker.pos.y][walker.pos.x].1 = CellType::Pipe;
    while walker.pos != Pos::new(start_x, start_y) {
        let new_pos = find_next(&field, &walker);
        field[new_pos.y][new_pos.x].1 = CellType::Pipe;
        walker.move_to(&new_pos);
    }
    walker.rotating = false;

    let mut inside_queue = VecDeque::new();

    walker.move_to(&find_next(&field, &walker));
    field[walker.pos.y][walker.pos.x].1 = CellType::Pipe;
    mark_along_path(&mut field, &walker, &mut inside_queue);
    while walker.pos != Pos::new(start_x, start_y) {
        let new_pos = find_next(&field, &walker);
        field[new_pos.y][new_pos.x].1 = CellType::Pipe;
        mark_along_path(&mut field, &walker, &mut inside_queue);
        walker.move_to(&new_pos);
    }
    mark_along_path(&mut field, &walker, &mut inside_queue);

    spread(&mut field, &mut inside_queue, &CellType::Inside);

    field
        .iter()
        .map(|row| row.iter().filter(|c| c.1 == CellType::Inside).count())
        .sum()
}
