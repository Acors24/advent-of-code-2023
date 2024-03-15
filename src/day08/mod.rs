use std::collections::HashMap;

pub fn part1(path: &str) -> i32 {
    let input = std::fs::read_to_string(path).unwrap();
    let (instructions, node_lines) = input.split_once("\n\n").unwrap();

    let mut map = HashMap::new();
    for line in node_lines.split('\n') {
        let (node, pair) = line.split_once(" = ").unwrap();
        let (left, right) = pair
            .trim_matches(|c| c == '(' || c == ')')
            .split_once(", ")
            .unwrap();
        map.insert(node, (left, right));
    }

    let (mut current_node, mut choices) = map.get_key_value("AAA").unwrap();
    let mut it = instructions.chars().cycle();
    let mut steps = 0;
    while current_node != &"ZZZ" {
        match it.next().unwrap() {
            'L' => (current_node, choices) = map.get_key_value(choices.0).unwrap(),
            'R' => (current_node, choices) = map.get_key_value(choices.1).unwrap(),
            _ => panic!("😳"),
        }
        steps += 1;
    }

    steps
}

fn gcd(a: &u64, b: &u64) -> u64 {
    if b == &0 {
        return *a;
    }

    gcd(b, &a.rem_euclid(*b))
}

fn lcm(a: &u64, b: &u64) -> u64 {
    a * b / gcd(a, b)
}

pub fn part2(path: &str) -> u64 {
    let input = std::fs::read_to_string(path).unwrap();
    let (instructions, node_lines) = input.split_once("\n\n").unwrap();

    let mut map = HashMap::new();
    for line in node_lines.split('\n') {
        let (node, pair) = line.split_once(" = ").unwrap();
        let (left, right) = pair
            .trim_matches(|c| c == '(' || c == ')')
            .split_once(", ")
            .unwrap();
        map.insert(node, (left, right));
    }

    let mut steps_vec = vec![];
    map.iter()
        .filter_map(|(k, _)| if k.ends_with('A') { Some(k) } else { None })
        .for_each(|node| {
            let mut it = instructions.chars().cycle();
            let mut node = node.to_owned();
            let mut steps = 0u64;
            while !node.ends_with('Z') {
                match it.next().unwrap() {
                    'L' => node = &map.get(node).unwrap().0,
                    'R' => node = &map.get(node).unwrap().1,
                    _ => panic!("😳"),
                }
                steps += 1;
            }
            steps_vec.push(steps);
        });
    steps_vec.iter().fold(1, |acc, e| lcm(&acc, e))
}
