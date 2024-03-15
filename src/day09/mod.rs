fn predict(lines: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for history in lines {
        let mut lines = Vec::new();
        lines.push(history.clone());
        while lines.last().unwrap().iter().any(|e| e != &0) {
            let mut derivative = Vec::new();
            for pair in lines.last().unwrap().windows(2) {
                derivative.push(pair.iter().rev().copied().reduce(i32::wrapping_sub).unwrap());
            }
            lines.push(derivative);
        }
        sum += lines.iter().rev().map(|line| line.last().copied().unwrap()).reduce(i32::wrapping_add).unwrap();
    }

    sum
}

pub fn part1(path: &str) -> i32 {
    let input = std::fs::read_to_string(path).unwrap();
    let lines = input.split('\n').map(|line| {
        line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    predict(&lines)
}

pub fn part2(path: &str) -> i32 {
    let input = std::fs::read_to_string(path).unwrap();
    let lines = input.split('\n').map(|line| {
        line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).rev().collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    predict(&lines)
}
