use regex::Regex;

pub fn part1(path: &str) -> usize {
    let input = std::fs::read_to_string(path).unwrap();
    let games: Vec<_> = input
        .split('\n')
        .map(|line| line.split_once(": ").unwrap().1)
        .map(|game| game.split("; ").collect::<Vec<_>>())
        .collect();

    let red_re = Regex::new("([0-9]+) red").unwrap();
    let green_re = Regex::new("([0-9]+) green").unwrap();
    let blue_re = Regex::new("([0-9]+) blue").unwrap();

    let res = games
        .iter()
        .map(|game| {
            game.iter()
                .map(|set| {
                    let red_cap = red_re.find(set);
                    if let Some(red) = red_cap {
                        if red
                            .as_str()
                            .split_once(' ')
                            .unwrap()
                            .0
                            .parse::<i32>()
                            .unwrap()
                            > 12
                        {
                            return false;
                        }
                    }
                    let green_cap = green_re.find(set);
                    if let Some(green) = green_cap {
                        if green
                            .as_str()
                            .split_once(' ')
                            .unwrap()
                            .0
                            .parse::<i32>()
                            .unwrap()
                            > 13
                        {
                            return false;
                        }
                    }
                    let blue_cap = blue_re.find(set);
                    if let Some(blue) = blue_cap {
                        if blue
                            .as_str()
                            .split_once(' ')
                            .unwrap()
                            .0
                            .parse::<i32>()
                            .unwrap()
                            > 14
                        {
                            return false;
                        }
                    }

                    return true;
                })
                .all(|arg| arg)
        })
        .enumerate()
        .map(|(i, b)| if b { i + 1 } else { 0 })
        .sum();

    res
}

pub fn part2(path: &str) -> i32 {
    let input = std::fs::read_to_string(path).unwrap();
    let games: Vec<_> = input
        .split('\n')
        .map(|line| line.split_once(": ").unwrap().1)
        .map(|game| game.split("; ").collect::<Vec<_>>())
        .collect();

    let red_re = Regex::new("([0-9]+) red").unwrap();
    let green_re = Regex::new("([0-9]+) green").unwrap();
    let blue_re = Regex::new("([0-9]+) blue").unwrap();

    let res = games
        .iter()
        .map(|game| {
            let mut red_min = 0;
            let mut green_min = 0;
            let mut blue_min = 0;

            game.iter()
                .for_each(|set| {
                    let red_cap = red_re.find(set);
                    if let Some(red) = red_cap {
                        let red_amount = red
                            .as_str()
                            .split_once(' ')
                            .unwrap()
                            .0
                            .parse::<i32>()
                            .unwrap();
                        if red_amount > red_min {
                            red_min = red_amount;
                        }
                    }
                    let green_cap = green_re.find(set);
                    if let Some(green) = green_cap {
                        let green_amount = green
                            .as_str()
                            .split_once(' ')
                            .unwrap()
                            .0
                            .parse::<i32>()
                            .unwrap();
                        if green_amount > green_min {
                            green_min = green_amount;
                        }
                    }
                    let blue_cap = blue_re.find(set);
                    if let Some(blue) = blue_cap {
                        let blue_amount = blue
                            .as_str()
                            .split_once(' ')
                            .unwrap()
                            .0
                            .parse::<i32>()
                            .unwrap();
                        if blue_amount > blue_min {
                            blue_min = blue_amount;
                        }
                    }
                });
            
            red_min * green_min * blue_min
        }).sum();

    res
}
