use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Brick {
    start: (i32, i32, i32),
    end: (i32, i32, i32),
    supports: HashSet<usize>,
    supported_by: HashSet<usize>,
}

fn intersects(area: ((i32, i32, i32), (i32, i32, i32)), brick: &Brick) -> bool {
    area.0.0 <= brick.end.0 && area.1.0 >= brick.start.0 &&
    area.0.1 <= brick.end.1 && area.1.1 >= brick.start.1 &&
    area.0.2 <= brick.end.2 && area.1.2 >= brick.start.2
}

fn intersects_any(area: ((i32, i32, i32), (i32, i32, i32)), bricks: &Vec<Brick>) -> bool {
    bricks.iter().any(|brick| intersects(area, brick))
}

fn fall(bricks: &mut Vec<Brick>) {
    bricks.sort_unstable_by_key(|brick| brick.start.2);

    for i in 0..bricks.len() {
        let mut brick = bricks[i].clone();
        let mut area_underneath = (
            (brick.start.0, brick.start.1, brick.start.2),
            (brick.end.0, brick.end.1, brick.start.2),
        );

        while brick.start.2 > 1 && brick.end.2 > 1 && !intersects_any(area_underneath, bricks) {
            brick.start.2 -= 1;
            brick.end.2 -= 1;
            area_underneath.0 .2 -= 1;
            area_underneath.1 .2 -= 1;
            // println!("fell");
        }

        bricks[i] = brick;
    }
}

pub fn part1(path: &str) -> i32 {
    let input = std::fs::read_to_string(path).unwrap();
    let mut bricks = input
        .split_whitespace()
        .map(|line| {
            let (start, end) = line.split_once('~').unwrap();
            if let [x1, y1, z1] = start
                .split(',')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>()[..]
            {
                if let [x2, y2, z2] = end
                    .split(',')
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()[..]
                {
                    Brick {
                        start: (x1, y1, z1),
                        end: (x2, y2, z2),
                        supports: HashSet::new(),
                        supported_by: HashSet::new(),
                    }
                } else {
                    panic!()
                }
            } else {
                panic!()
            }
        })
        .collect::<Vec<_>>();

    fall(&mut bricks);

    // println!("{:#?}", bricks);

    -1
}
