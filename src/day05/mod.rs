pub fn part1(path: &str) -> i64 {
    let input = std::fs::read_to_string(path).unwrap();
    let (seed_line, map_section) = input.split_once("\n\n").unwrap();

    let seeds: Vec<i64> = seed_line
        .split_once(" ")
        .unwrap()
        .1
        .split(" ")
        .map(|num_str| num_str.parse::<i64>().unwrap())
        .collect();
    let map_groups = map_section
        .split("\n\n")
        .map(|subsection| {
            subsection
                .split_once("\n")
                .unwrap()
                .1
                .split("\n")
                .map(|map_str| {
                    map_str
                        .split(" ")
                        .map(|num_str| num_str.parse::<i64>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    seeds
        .iter()
        .map(|seed| {
            let mut value = seed.clone();
            for map_group in map_groups.iter() {
                for map in map_group.iter() {
                    let dst_start = map[0];
                    let src_start = map[1];
                    let range_len = map[2];

                    if (src_start..src_start + range_len).contains(&value) {
                        let offset = value - src_start;
                        value = dst_start + offset;
                        break;
                    }
                }
            }

            value
        })
        .min()
        .unwrap()
}

pub fn part2(path: &str) -> i64 {
    let input = std::fs::read_to_string(path).unwrap();
    let (seed_line, map_section) = input.split_once("\n\n").unwrap();

    let seed_data: Vec<i64> = seed_line
        .split_once(" ")
        .unwrap()
        .1
        .split(" ")
        .map(|num_str| num_str.parse::<i64>().unwrap())
        .collect();
    let seed_ranges = seed_data.chunks(2).map(|chunk| {
        let range_start = chunk.get(0).unwrap().clone();
        let range_length = chunk.get(1).unwrap().clone();

        range_start..range_start + range_length
    }).collect::<Vec<_>>();

    let map_groups = map_section
        .split("\n\n")
        .map(|subsection| {
            subsection
                .split_once("\n")
                .unwrap()
                .1
                .split("\n")
                .map(|map_str| {
                    map_str
                        .split(" ")
                        .map(|num_str| num_str.parse::<i64>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    seed_ranges
        .iter()
        .map(|range| {
            range.clone().map(|seed| {
                let mut value = seed.clone();
                for map_group in map_groups.iter() {
                    for map in map_group.iter() {
                        let dst_start = map[0];
                        let src_start = map[1];
                        let range_len = map[2];
    
                        if (src_start..src_start + range_len).contains(&value) {
                            let offset = value - src_start;
                            value = dst_start + offset;
                            break;
                        }
                    }
                }

                value
            }).min().unwrap()
        })
        .min()
        .unwrap()
}
