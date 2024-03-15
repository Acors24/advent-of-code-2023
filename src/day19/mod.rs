use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Rule {
    category: char,
    op: char,
    num: i32,
    next: String,
}

impl Rule {
    fn new(category: char, op: char, num: i32, next: &str) -> Self {
        Self {
            category,
            op,
            num,
            next: next.to_string(),
        }
    }

    fn eval(&self, num: i32) -> bool {
        match self.op {
            '<' => num < self.num,
            '>' => num > self.num,
            _ => panic!(),
        }
    }
}

fn parse_rule(s: &str) -> Rule {
    if let Some((condition, next)) = s.split_once(':') {
        let mut iter = condition.chars();
        let category = iter.next().unwrap();
        let op = iter.next().unwrap();
        let num = iter.collect::<String>().parse::<i32>().unwrap();

        Rule::new(category, op, num, next)
    } else {
        Rule::new('*', '>', 0, s)
    }
}

fn parse_workflow(s: &str) -> (&str, Vec<Rule>) {
    let (name, rest) = s.split_once('{').unwrap();
    let rest_len = rest.len();
    let rules = &rest[..rest_len - 1]
        .split(',')
        .map(parse_rule)
        .collect::<Vec<_>>();

    (name, rules.to_vec())
}

fn parse_item(s: &str) -> HashMap<char, i32> {
    let len = s.len();
    let mut ratings = HashMap::new();
    s[1..len-1].split(',').for_each(|v| {
        let (category, rating) = v.split_once('=').unwrap();
        ratings.insert(category.chars().next().unwrap(), rating.parse::<i32>().unwrap());
    });
    
    ratings
}

enum Result {
    Accepted,
    Rejected,
    Next(String)
}

fn eval_item(item: &HashMap<char, i32>, workflow: &Vec<Rule>) -> Result {
    dbg!(item);
    for rule in workflow {
        dbg!(rule);
        if rule.category == '*' || rule.eval(*item.get(&rule.category).unwrap()) {
            match &rule.next[..] {
                "A" => return Result::Accepted,
                "R" => return Result::Rejected,
                other => return Result::Next(other.to_string())
            }
        }
    }

    Result::Next(workflow.last().unwrap().next.to_string())
}

pub fn part1(path: &str) -> i32 {
    let input = std::fs::read_to_string(path).unwrap();
    let data = input.split_once("\n\n").unwrap();
    let mut workflows = HashMap::new();
    data.0.split_ascii_whitespace().for_each(|line| {
        let (name, rules) = parse_workflow(line);
        workflows.insert(name, rules);
    });

    let items = data.1.split_whitespace().map(parse_item).collect::<Vec<_>>();

    let mut accepted_sum = 0;

    for item in &items {
        let mut workflow = workflows.get("in").unwrap();
        loop {
            match eval_item(item, workflow) {
                Result::Accepted => {
                    accepted_sum += item.values().sum::<i32>();
                    break;
                },
                Result::Rejected => break,
                Result::Next(next) => workflow = workflows.get(&next[..]).unwrap()
            }
        }
    }

    accepted_sum
}

fn intersect(a1: &HashMap<char, (i32, i32)>, a2: &HashMap<char, (i32, i32)>) -> HashMap<char, (i32, i32)> {
    let mut result = a1.clone();

    for category in result.clone().into_keys() {
        let (this_min, this_max) = result.get(&category).unwrap();
        let (other_min, other_max) = a2.get(&category).unwrap();

        result.insert(category, (*this_min.max(other_min), *this_max.min(other_max)));
    }

    result
}

fn check_workflow(workflow_name: &str, accepted: &HashMap<char, (i32, i32)>, workflows: &HashMap<&str, Vec<Rule>>) -> HashMap<char, (i32, i32)> {
    let mut accepted = accepted.to_owned();
    let empty = HashMap::from([
        ('x', (0, 0)),
        ('m', (0, 0)),
        ('a', (0, 0)),
        ('s', (0, 0)),
    ]);

    for rule in workflows.get(workflow_name).unwrap() {
        match &rule.next[..] {
            "R" => {
                // exclude
                let category = &rule.category;
                if category == &'*' {
                    return empty;
                } else {
                    let (min, max) = accepted.get(category).unwrap();
                    match rule.op {
                        '<' => accepted.insert(*category, (*min.min(&rule.num), *max)),
                        '>' => accepted.insert(*category, (*min, *max.max(&rule.num))),
                        _ => panic!()
                    };
                }
                // continue
            },
            "A" => {
                let category = &rule.category;
                if category == &'*' {
                    dbg!(workflow_name, &accepted);
                    return accepted;
                } else {
                    let (min, max) = accepted.get(category).unwrap();
                    match rule.op {
                        '<' => accepted.insert(*category, (*min, *max.min(&rule.num))),
                        '>' => accepted.insert(*category, (*min.max(&rule.num), *max)),
                        _ => panic!()
                    };
                }
                // continue
            },
            other => {
                // recurse
                // intersect
                accepted = intersect(&accepted, &check_workflow(other, &accepted, workflows));
            }
        }
    }

    dbg!(workflow_name, &accepted);
    accepted
}

pub fn part2(path: &str) -> i32 {
    let input = std::fs::read_to_string(path).unwrap();
    let data = input.split_once("\n\n").unwrap();
    let mut workflows = HashMap::new();
    data.0.split_ascii_whitespace().for_each(|line| {
        let (name, rules) = parse_workflow(line);
        workflows.insert(name, rules);
    });

    let full = HashMap::from([
        ('x', (1, 4000)),
        ('m', (1, 4000)),
        ('a', (1, 4000)),
        ('s', (1, 4000)),
    ]);

    // println!("{:#?}", check_workflow("lnx", &full, &workflows));
    
    // let items = data.1.split_whitespace().map(parse_item).collect::<Vec<_>>();

    // // println!("{:#?}", items);

    // let mut accepted_sum = 0;

    // for item in &items {
    //     let mut workflow = workflows.get("in").unwrap();
    //     loop {
    //         match eval_item(item, workflow) {
    //             Result::Accepted => {
    //                 accepted_sum += item.values().sum::<i32>();
    //                 break;
    //             },
    //             Result::Rejected => break,
    //             Result::Next(next) => workflow = workflows.get(&next[..]).unwrap()
    //         }
    //     }
    // }

    // accepted_sum
    
    -1
}
