use std::collections::HashSet;

pub fn part1(path: &str) -> usize {
    let input = std::fs::read_to_string(path).unwrap();
    let lines = input.split("\n").collect::<Vec<_>>();
    let cards = lines.iter().map(|line| line.split_once(": ").unwrap().1.split_once(" | ").unwrap()).collect::<Vec<_>>();

    let results = cards.iter().map(|(winning_part, my_part)| {
        let winning_set: HashSet<i32> = HashSet::from_iter(winning_part.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>());
        let my_set: HashSet<i32> = HashSet::from_iter(my_part.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>());

        match winning_set.intersection(&my_set).count() {
            0 => 0,
            n => 2usize.pow((n - 1).try_into().unwrap())
        }
    }).collect::<Vec<_>>();

    results.iter().sum()
}

pub fn part2(path: &str) -> usize {
    let input = std::fs::read_to_string(path).unwrap();
    let lines = input.split("\n").collect::<Vec<_>>();
    let cards = lines.iter().map(|line| line.split_once(": ").unwrap().1.split_once(" | ").unwrap()).collect::<Vec<_>>();
    let mut amounts = vec![1; cards.len()];

    cards.iter().enumerate().for_each(|(i, (winning_part, my_part))| {
        let winning_set: HashSet<i32> = HashSet::from_iter(winning_part.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>());
        let my_set: HashSet<i32> = HashSet::from_iter(my_part.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>());

        let won = winning_set.intersection(&my_set).count();

        if won != 0 {
            let current_amount = amounts.get(i).unwrap().clone();
            amounts.get_mut(i+1..=i+won).unwrap().iter_mut().for_each(|amount| { *amount += current_amount; } );
        }
    });

    amounts.iter().sum()
}
