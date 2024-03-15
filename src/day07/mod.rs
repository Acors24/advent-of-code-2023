use std::{
    cmp::Ordering,
    collections::HashMap, ops::AddAssign,
};

#[derive(Debug)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn get_type(set: &HashMap<char, i32>) -> Type {
    let mut amounts = Vec::from_iter(set.values());
    amounts.sort();
    match amounts[..] {
        [1, 1, 1, 1, 1] => Type::HighCard,
        [1, 1, 1, 2] => Type::OnePair,
        [1, 2, 2] => Type::TwoPair,
        [1, 1, 3] => Type::ThreeOfAKind,
        [2, 3] => Type::FullHouse,
        [1, 4] => Type::FourOfAKind,
        [5] => Type::FiveOfAKind,
        _ => panic!("{amounts:#?}"),
    }
}

fn compare1(a: &(&str, i32), b: &(&str, i32)) -> Ordering {
    let a =
        a.0.chars()
            .map(|ch| match ch {
                'A' => 'E',
                'K' => 'D',
                'Q' => 'C',
                'J' => 'B',
                'T' => 'A',
                other => other,
            })
            .collect::<String>();

    let b =
        b.0.chars()
            .map(|ch| match ch {
                'A' => 'E',
                'K' => 'D',
                'Q' => 'C',
                'J' => 'B',
                'T' => 'A',
                other => other,
            })
            .collect::<String>();

    a.cmp(&b)
}

fn compare2(a: &(&str, i32), b: &(&str, i32)) -> Ordering {
    let a =
        a.0.chars()
            .map(|ch| match ch {
                'A' => 'E',
                'K' => 'D',
                'Q' => 'C',
                'J' => '1',
                'T' => 'A',
                other => other,
            })
            .collect::<String>();

    let b =
        b.0.chars()
            .map(|ch| match ch {
                'A' => 'E',
                'K' => 'D',
                'Q' => 'C',
                'J' => '1',
                'T' => 'A',
                other => other,
            })
            .collect::<String>();

    a.cmp(&b)
}

pub fn part1(path: &str) -> i32 {
    let input = std::fs::read_to_string(path).unwrap();
    let hands = input
        .split('\n')
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            (hand, bid.parse::<i32>().unwrap())
        })
        .collect::<Vec<_>>();

    let mut data: Vec<Vec<(&str, i32)>> = vec![vec![]; 7];

    for hand in hands.iter() {
        let mut set: HashMap<char, i32> = HashMap::new();
        for ch in hand.0.to_string().chars() {
            if let Some(v) = set.get_mut(&ch) {
                *v += 1;
            } else {
                set.insert(ch, 1);
            }
        }

        data[get_type(&set) as usize].push(*hand);
    }

    data.iter_mut().for_each(|thing| thing.sort_by(compare1));
    let data = data.iter().flatten().collect::<Vec<_>>();

    data.iter()
        .enumerate()
        .map(|(rank, (_, bid))| (rank as i32 + 1) * bid)
        .sum::<i32>()
}

pub fn part2(path: &str) -> i32 {
    let input = std::fs::read_to_string(path).unwrap();
    let hands = input
        .split('\n')
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            (hand, bid.parse::<i32>().unwrap())
        })
        .collect::<Vec<_>>();

    let mut data: Vec<Vec<(&str, i32)>> = vec![vec![]; 7];

    for hand in hands.iter() {
        let mut js = 0;
        let mut set: HashMap<char, i32> = HashMap::new();
        for ch in hand.0.to_string().chars() {
            if ch == 'J' {
                js += 1;
            } else if let Some(v) = set.get_mut(&ch) {
                *v += 1;
            } else {
                set.insert(ch, 1);
            }
        }
        match set.iter_mut().max_by_key(|pair| *pair.1) {
            Some(v) => v.1.add_assign(js),
            None => {
                set = HashMap::from([('J', js)]);
            },
        }
        data[get_type(&set) as usize].push(*hand);
    }

    data.iter_mut().for_each(|thing| thing.sort_by(compare2));
    let data = data.iter().flatten().collect::<Vec<_>>();

    data.iter()
        .enumerate()
        .map(|(rank, (_, bid))| (rank as i32 + 1) * bid)
        .sum::<i32>()
}
