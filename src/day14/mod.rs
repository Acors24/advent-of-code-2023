fn roll_north(x: usize, y: usize, platform: &mut Vec<Vec<char>>) -> i64 {
    if y == 0 || "#O".contains(platform[y - 1][x]) {
        return (platform.len() - y) as i64;
    }

    platform[y][x] = '.';
    platform[y - 1][x] = 'O';
    roll_north(x, y - 1, platform)
}

fn roll_south(x: usize, y: usize, platform: &mut Vec<Vec<char>>) -> i64 {
    if y == platform.len() - 1 || "#O".contains(platform[y + 1][x]) {
        return (platform.len() - y) as i64;
    }

    platform[y][x] = '.';
    platform[y + 1][x] = 'O';
    roll_south(x, y + 1, platform)
}

fn roll_west(x: usize, y: usize, platform: &mut Vec<Vec<char>>) -> i64 {
    if x == 0 || "#O".contains(platform[y][x - 1]) {
        return (platform.len() - y) as i64;
    }

    platform[y][x] = '.';
    platform[y][x - 1] = 'O';
    roll_west(x - 1, y, platform)
}

fn roll_east(x: usize, y: usize, platform: &mut Vec<Vec<char>>) -> i64 {
    if x == platform[0].len() - 1 || "#O".contains(platform[y][x + 1]) {
        return (platform.len() - y) as i64;
    }

    platform[y][x] = '.';
    platform[y][x + 1] = 'O';
    roll_east(x + 1, y, platform)
}

fn roll_all_north(platform: &mut Vec<Vec<char>>) -> i64 {
    let width = platform[0].len();
    let height = platform.len();
    let mut sum = 0i64;

    for y in 0..height {
        for x in 0..width {
            if platform[y][x] == 'O' {
                sum += roll_north(x, y, platform);
            }
        }
    }

    sum
}

fn roll_all_south(platform: &mut Vec<Vec<char>>) -> i64 {
    let width = platform[0].len();
    let height = platform.len();
    let mut sum = 0i64;

    for y in (0..height).rev() {
        for x in 0..width {
            if platform[y][x] == 'O' {
                sum += roll_south(x, y, platform);
            }
        }
    }

    sum
}

fn roll_all_west(platform: &mut Vec<Vec<char>>) -> i64 {
    let width = platform[0].len();
    let height = platform.len();
    let mut sum = 0i64;

    for y in 0..height {
        for x in 0..width {
            if platform[y][x] == 'O' {
                sum += roll_west(x, y, platform);
            }
        }
    }

    sum
}

fn roll_all_east(platform: &mut Vec<Vec<char>>) -> i64 {
    let width = platform[0].len();
    let height = platform.len();
    let mut sum = 0i64;

    for y in 0..height {
        for x in (0..width).rev() {
            if platform[y][x] == 'O' {
                sum += roll_east(x, y, platform);
            }
        }
    }

    sum
}

pub fn part1(path: &str) -> i64 {
    let input = std::fs::read_to_string(path).unwrap();
    let mut platform = input
        .split_ascii_whitespace()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    roll_all_north(&mut platform)
}

pub fn part2(path: &str) -> i64 {
    let input = std::fs::read_to_string(path).unwrap();
    let mut platform = input
        .split_ascii_whitespace()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut res = 0;
    for i in 0..1_000_000_000 {
        roll_all_north(&mut platform);
        roll_all_west(&mut platform);
        roll_all_south(&mut platform);
        res = roll_all_east(&mut platform);
        // println!("{i}: {res}");
        // i % 13 =  5: 87258
        // i % 13 =  6: 87272
        // i % 13 =  7: 87286
        // i % 13 =  8: 87288
        // i % 13 =  9: 87271
        // i % 13 = 10: 87266
        // i % 13 = 11: 87273
        // i % 13 = 12: 87287
        // i % 13 =  0: 87292
        // i % 13 =  1: 87286
        // i % 13 =  2: 87284
        // i % 13 =  3: 87282
        // i % 13 =  4: 87264

        // 999999999 % 13 = 11 => 87273
    }

    res
}
