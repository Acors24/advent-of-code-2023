pub fn part1(path: &str) -> f64 {
    let input = std::fs::read_to_string(path).unwrap();
    let (time_line, distance_line) = input.split_once('\n').unwrap();
    let extract = |line: &str| {
        line.split_whitespace()
            .skip(1)
            .map(|str| str.parse::<f64>().unwrap())
            .collect::<Vec<_>>()
    };
    let times = extract(time_line);
    let distances = extract(distance_line);

    times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| {
            let a = -1f64;

            let d = time * time - 4f64 * distance;
            let x0 = ((time + d.sqrt()) / 2f64 / a + 1f64).floor();
            let x1 = ((time - d.sqrt()) / 2f64 / a - 1f64).ceil();

            (x0 - x1).abs() + 1f64
        })
        .product()
}

pub fn part2(path: &str) -> f64 {
    let input = std::fs::read_to_string(path).unwrap();
    let (time_line, distance_line) = input.split_once('\n').unwrap();
    let extract = |line: &str| {
        line.split_whitespace()
            .skip(1)
            .collect::<Vec<_>>()
            .join("")
            .parse::<f64>()
            .unwrap()
    };
    let time = extract(time_line);
    let distance = extract(distance_line);

    let a = -1f64;

    let d = time * time - 4f64 * distance;
    let x0 = ((time + d.sqrt()) / 2f64 / a + 1f64).floor();
    let x1 = ((time - d.sqrt()) / 2f64 / a - 1f64).ceil();

    (x0 - x1).abs() + 1f64
}
