fn hash(s: &str) -> i32 {
    s.chars()
        .fold(0, |acc: u8, c| acc.wrapping_add(c as u8).wrapping_mul(17)) as i32
}

pub fn part1(path: &str) -> i32 {
    let input = std::fs::read_to_string(path).unwrap();
    let sequence = input.split(",").collect::<Vec<_>>();

    sequence.into_iter().map(hash).sum()
}

pub fn part2(path: &str) -> i32 {
    let input = std::fs::read_to_string(path).unwrap();
    let sequence = input
        .split(",")
        .map(|s| {
            if let Some(res) = s.split_once('=') {
                res
            } else {
                s.split_once('-').unwrap()
            }
        })
        .collect::<Vec<_>>();

    let mut boxes: Vec<Vec<(&str, i32)>> = vec![vec![]; 256];

    sequence.iter().for_each(|(label, focal_length)| {
        let box_ = hash(label) as usize;
        let pos = boxes[box_]
            .iter()
            .position(|(other_label, _)| other_label == label);
        match focal_length {
            &"" => {
                if let Some(pos) = pos {
                    boxes[box_].remove(pos);
                }
            }
            _ => {
                let focal_length = focal_length.parse::<i32>().unwrap();
                if let Some(pos) = pos {
                    boxes[box_][pos] = (label, focal_length);
                } else {
                    boxes[box_].push((label, focal_length));
                }
            }
        }
    });

    boxes
        .iter()
        .enumerate()
        .map(|(i, box_)| {
            box_.iter()
                .enumerate()
                .map(move |(slot, (_, focal_length))| {
                    (i + 1) as i32 * (slot + 1) as i32 * focal_length
                })
                .sum::<i32>()
        })
        .sum()
}
