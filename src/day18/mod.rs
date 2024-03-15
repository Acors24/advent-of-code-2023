pub fn part1(path: &str) -> i32 {
    let input = std::fs::read_to_string(path).unwrap();
    let lines = input
        .lines()
        .map(|line| {
            let v = line.split_whitespace().collect::<Vec<_>>();
            (v[0], v[1].parse::<i32>().unwrap(), v[2])
        })
        .collect::<Vec<_>>();

    let mut pos = (0, 0);
    let mut border_length = 0;
    let mut points = Vec::new();

    for line in &lines {
        let (dir, len, _) = line;
        pos = match *dir {
            "U" => (pos.0, pos.1 - len),
            "D" => (pos.0, pos.1 + len),
            "L" => (pos.0 - len, pos.1),
            "R" => (pos.0 + len, pos.1),
            _ => panic!(),
        };
        points.push(pos);
        border_length += len;
    }
    
    let mut area = 0;
    points.push(points.first().unwrap().clone());

    for a in points.windows(2) {
        if let [(x1, y1), (x2, y2)] = a {
            area += x1 * y2 - y1 * x2;
        }
    }

    area / 2 + border_length / 2 + 1
}

fn hex_to_dec(hex: &str) -> i64 {
    let len = hex.len() as u32;
    hex.char_indices().map(|(i, c)| 16i64.pow(len - i as u32 - 1) * match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'a' => 10,
        'b' => 11,
        'c' => 12,
        'd' => 13,
        'e' => 14,
        'f' => 15,
        _ => panic!()
    }).sum()
}

pub fn part2(path: &str) -> i64 {
    let input = std::fs::read_to_string(path).unwrap();
    let lines = input
        .lines()
        .map(|line| {
            let third = line.split_whitespace().last().unwrap();
            third.chars().skip(2).take(6).collect::<String>()
        })
        .collect::<Vec<_>>();

    let mut pos = (0, 0);
    let mut border_length = 0;
    let mut points = Vec::new();

    for line in &lines {
        let len = hex_to_dec(&line[..5]);
        let dir = &line[5..].to_string().parse::<i8>().unwrap();
        pos = match dir {
            3 => (pos.0, pos.1 - len),
            1 => (pos.0, pos.1 + len),
            2 => (pos.0 - len, pos.1),
            0 => (pos.0 + len, pos.1),
            _ => panic!(),
        };
        points.push(pos);
        border_length += len;
    }

    let mut area = 0;
    points.push(points.first().unwrap().clone());

    // println!("{points:?}");
    for a in points.windows(2) {
        if let [(x1, y1), (x2, y2)] = a {
            area += x1 * y2 - y1 * x2;
        }
    }

    area / 2 + border_length / 2 + 1
}
