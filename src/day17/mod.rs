use std::{cmp::Reverse, collections::BinaryHeap};

struct Graph {
    width: usize,
    height: usize,
    nodes: Vec<i32>,
    edges: Vec<Vec<usize>>,
    cell_size: usize,
    min_distance: usize,
    max_distance: usize,
}

impl Graph {
    pub fn new(grid: &Vec<Vec<i32>>, min_distance: usize, max_distance: usize) -> Self {
        let cell_size = 4 * max_distance;
        let mut graph = Graph {
            width: grid[0].len(),
            height: grid.len(),
            nodes: Vec::new(),
            edges: vec![Vec::new(); grid.len() * grid[0].len() * cell_size],
            cell_size,
            min_distance,
            max_distance,
        };
        
        for row in grid.iter() {
            for cell in row.iter() {
                for _ in 0..graph.cell_size {
                    graph.add_node(*cell);
                    }
            }
        }
        
        let width = graph.width;
        let height = graph.height;

        for y in 0..height {
            for x in 0..width {
                for i in 0..graph.cell_size {
                    let index = graph.get_index(x, y, i);
                    graph.link_right(index);
                    graph.link_left(index);
                    graph.link_down(index);
                    graph.link_up(index);
                }
            }
        }

        graph
    }

    fn add_node(&mut self, node: i32) {
        self.nodes.push(node);
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.edges[from].push(to);
    }

    fn get_index(&self, x: usize, y: usize, z: usize) -> usize {
        (y * self.width + x) * self.cell_size + z
    }

    fn get_coords(&self, index: usize) -> (usize, usize, usize) {
        let x = index / self.cell_size % self.width;
        let y = index / self.cell_size / self.width;
        let z = index % self.cell_size;
        (x, y, z)
    }

    fn get_neighbors(&self, index: usize) -> Vec<usize> {
        self.edges.get(index).unwrap().clone()
    }

    fn get_cost(&self, index: usize) -> i32 {
        self.nodes[index]
    }

    fn in_ranges(x: usize, x1_min: usize, x1_max: usize, x2_min: usize, x2_max: usize) -> bool {
        (x1_min..=x1_max).contains(&x) || (x2_min..=x2_max).contains(&x)
    }

    fn link_right(&mut self, index: usize) {
        let (x, y, z) = self.get_coords(index);
        let z1_min = self.max_distance + self.min_distance - 1;
        let z1_max = self.max_distance + self.max_distance - 1;
        let z2_min = self.max_distance * 3 + self.min_distance - 1;
        let z2_max = self.max_distance * 3 + self.max_distance - 1;
        if x >= self.width - self.min_distance
            || !Graph::in_ranges(z, z1_min, z1_max, z2_min, z2_max)
        {
            return;
        }

        let root = self.get_index(x + 1, y, 0);
        self.add_edge(index, root);

        if self.get_neighbors(root).contains(&self.get_index(x + 1, y, 0)) {
            return;
        }

        let width = self.width;
        let xs = (x + 1..x + self.max_distance + 1)
            .filter(|&x| x < width)
            .collect::<Vec<_>>();
        let xs = xs.windows(2).enumerate();

        for (i, win) in xs {
            let (x, next_x) = (win[0], win[1]);
            self.add_edge(self.get_index(x, y, i), self.get_index(next_x, y, i + 1));
        }
    }

    fn link_down(&mut self, index: usize) {
        let (x, y, z) = self.get_coords(index);
        let z1_min = self.max_distance * 2 + self.min_distance - 1;
        let z1_max = self.max_distance * 2 + self.max_distance - 1;
        let z2_min = self.min_distance - 1;
        let z2_max = self.max_distance - 1;
        if y >= self.height - self.min_distance
            || !Graph::in_ranges(z, z1_min, z1_max, z2_min, z2_max)
        {
            return;
        }

        let root = self.get_index(x, y + 1, self.max_distance);
        self.add_edge(index, root);

        if self.get_neighbors(root).contains(&self.get_index(x, y + 1, 0)) {
            return;
        }

        let height = self.height;
        let ys = (y + 1..y + self.max_distance + 1)
            .filter(|&y| y < height)
            .collect::<Vec<_>>();
        let ys = ys.windows(2).enumerate();

        for (i, win) in ys {
            let i = i + self.max_distance;
            let (y, next_y) = (win[0], win[1]);
            self.add_edge(self.get_index(x, y, i), self.get_index(x, next_y, i + 1));
        }
    }

    fn link_left(&mut self, index: usize) {
        let (x, y, z) = self.get_coords(index);
        let z1_min = self.max_distance * 3 + self.min_distance - 1;
        let z1_max = self.max_distance * 3 + self.max_distance - 1;
        let z2_min = self.max_distance + self.min_distance - 1;
        let z2_max = self.max_distance + self.max_distance - 1;
        if x < self.min_distance || !Graph::in_ranges(z, z1_min, z1_max, z2_min, z2_max) {
            return;
        }

        let root = self.get_index(x - 1, y, self.max_distance * 2);
        self.add_edge(index, root);

        if self.get_neighbors(root).contains(&self.get_index(x - 1, y, 0)) {
            return;
        }

        let xs = (x.saturating_sub(self.max_distance)..x).collect::<Vec<_>>();
        let xs = xs.windows(2).rev().enumerate().rev();

        for (i, win) in xs {
            let i = i + self.max_distance * 2;
            let (next_x, x) = (win[0], win[1]);
            self.add_edge(self.get_index(x, y, i), self.get_index(next_x, y, i + 1));
        }
    }

    fn link_up(&mut self, index: usize) {
        let (x, y, z) = self.get_coords(index);
        let z1_min = self.min_distance - 1;
        let z1_max = self.max_distance - 1;
        let z2_min = self.max_distance * 2 + self.min_distance - 1;
        let z2_max = self.max_distance * 2 + self.max_distance - 1;
        if y < self.min_distance || !Graph::in_ranges(z, z1_min, z1_max, z2_min, z2_max) {
            return;
        }

        let root = self.get_index(x, y - 1, self.max_distance * 3);
        self.add_edge(index, root);

        if self.get_neighbors(root).contains(&self.get_index(x, y - 1, 0)) {
            return;
        }

        let ys = (y.saturating_sub(self.max_distance)..y).collect::<Vec<_>>();
        let ys = ys.windows(2).rev().enumerate().rev();

        for (i, win) in ys {
            let i = i + self.max_distance * 3;
            let (next_y, y) = (win[0], win[1]);
            self.add_edge(self.get_index(x, y, i), self.get_index(x, next_y, i + 1));
        }
    }
}

fn walk(graph: &Graph) -> i32 {
    let mut costs = vec![std::i32::MAX; graph.nodes.len()];
    let mut neighbors: BinaryHeap<(Reverse<i32>, usize)> = BinaryHeap::new();
    let mut sources: Vec<usize> = vec![0; graph.nodes.len()];

    {
        let from_down = graph.max_distance * 4 - 1;
        let from_right = graph.max_distance * 3 - 1;
        let right_neighbors = graph.get_neighbors(from_down);
        let down_neighbors = graph.get_neighbors(from_right);

        for n in right_neighbors.iter() {
            sources[*n] = from_down;
        }

        for n in down_neighbors.iter() {
            sources[*n] = from_right;
        }

        for &n in right_neighbors.iter().chain(down_neighbors.iter()) {
            neighbors.push((Reverse(graph.get_cost(n)), n));
        }
    }

    let mut visited = vec![false; graph.nodes.len()];
    for i in 0..graph.cell_size {
        visited[i] = true;
    }

    while let Some((Reverse(cost), node)) = neighbors.pop() {
        if visited[node] {
            continue;
        }
        visited[node] = true;

        for n in graph.get_neighbors(node) {
            let new_cost = cost + graph.get_cost(n);
            if new_cost < costs[n] {
                costs[n] = new_cost;
                neighbors.push((Reverse(new_cost), n));
                sources[n] = node;
            }
        }
    }

    let last_cell_indices = (graph.width * graph.height - 1) * graph.cell_size..graph.nodes.len();
    let last_cell_costs = costs[last_cell_indices.clone()].to_vec();

    let min_cost = *last_cell_costs.iter().min().unwrap();

    // print_path(last_cell_indices, &costs, &sources, graph, min_cost);

    min_cost
}

#[allow(dead_code)]
fn print_path(
    last_cell_indices: std::ops::Range<usize>,
    costs: &Vec<i32>,
    sources: &Vec<usize>,
    graph: &Graph,
    min_cost: i32,
) {
    for i in last_cell_indices {
        if costs[i] == min_cost {
            let mut path = vec![i];
            let mut current = i;
            while current >= graph.cell_size {
                current = sources[current];
                path.push(current);
            }
            let pairs = path
                .iter()
                .map(|index| {
                    let (x, y, _) = graph.get_coords(*index);
                    (x, y)
                })
                .collect::<Vec<_>>();
            for y in 0..graph.height {
                for x in 0..graph.width {
                    if pairs.iter().any(|(px, py)| *px == x && *py == y) {
                        print!("X");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
            println!();
        }
    }
}

fn get_grid(path: &str) -> Vec<Vec<i32>> {
    let input = std::fs::read_to_string(path).unwrap();
    input
        .split_ascii_whitespace()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn part1(path: &str) -> i32 {
    let graph = Graph::new(&get_grid(path), 1, 3);
    walk(&graph)
}

pub fn part2(path: &str) -> i32 {
    let graph = Graph::new(&get_grid(path), 4, 10);
    walk(&graph)
}
