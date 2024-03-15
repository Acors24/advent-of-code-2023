fn get_row_indices(image: &Vec<Vec<char>>) -> Vec<usize> {
    image
        .iter()
        .enumerate()
        .filter(|(_, line)| line.iter().all(|c| c == &'.'))
        .map(|(index, _)| index)
        .collect::<Vec<_>>()
}

fn get_col_indices(image: &Vec<Vec<char>>) -> Vec<usize> {
    image
        .iter()
        .map(|line| {
            line.iter()
                .enumerate()
                .filter(|(_, c)| *c == &'.')
                .map(|(index, _)| index)
                .collect::<Vec<_>>()
        })
        .reduce(|a, b| a.into_iter().filter(|a| b.contains(a)).collect())
        .unwrap()
}

fn expand(image: &mut Vec<Vec<char>>) {
    let width = image[0].len();
    for row_index in get_row_indices(image).into_iter().rev() {
        image.insert(row_index, ['.'].repeat(width));
    }

    let col_indices = get_col_indices(image);
    for row in image.into_iter() {
        for col_index in col_indices.iter().rev() {
            row.insert(*col_index, '.');
        }
    }
}

fn find_galaxies(image: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut galaxies = vec![];
    for (y, row) in image.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell == &'#' {
                galaxies.push((x, y));
            }
        }
    }

    galaxies
}

pub fn part1(path: &str) -> i64 {
    let input = std::fs::read_to_string(path).unwrap();
    let mut image = input
        .split_whitespace()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    expand(&mut image);
    let galaxies = find_galaxies(&image);
    let mut sum = 0;

    for (i, g1) in galaxies.iter().enumerate() {
        for g2 in galaxies[i + 1..].iter() {
            let dx = g1.0 as i64 - g2.0 as i64;
            let dy = g1.1 as i64 - g2.1 as i64;

            sum += dx.abs() + dy.abs();
        }
    }

    sum
}

pub fn part2(path: &str) -> i64 {
    let input = std::fs::read_to_string(path).unwrap();
    let image = input
        .split_whitespace()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let galaxies = find_galaxies(&image);
    let mut sum = 0;

    let minmax = |a: i64, b: i64| (a.min(b), a.max(b));

    let row_indices = get_row_indices(&image);
    let col_indices = get_col_indices(&image);
    let expansion = 999_999;
    for (i, g1) in galaxies.iter().enumerate() {
        for g2 in galaxies[i + 1..].iter() {
            let (min_x, max_x) = minmax(g1.0 as i64, g2.0 as i64);
            let (min_y, max_y) = minmax(g1.1 as i64, g2.1 as i64);
            let dx = max_x - min_x
                + expansion
                    * (min_x + 1..max_x)
                        .filter(|x| col_indices.contains(&(*x as usize)))
                        .count() as i64;
            let dy = max_y - min_y
                + expansion
                    * (min_y + 1..max_y)
                        .filter(|y| row_indices.contains(&(*y as usize)))
                        .count() as i64;

            sum += dx.abs() + dy.abs();
        }
    }

    sum
}
