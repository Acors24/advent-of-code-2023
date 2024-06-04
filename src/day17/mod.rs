use std::{
    cmp::Reverse,
    collections::BinaryHeap,
};

#[derive(PartialEq, Eq, Hash, Clone)]
struct Node {
    entry_cost: i32,
}

struct Graph {
    width: usize,
    height: usize,
    nodes: Vec<Node>,
    edges: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(grid: &Vec<Vec<i32>>) -> Self {
        let mut graph = Graph {
            width: grid[0].len(),
            height: grid.len(),
            nodes: Vec::new(),
            edges: vec![Vec::new(); grid.len() * grid[0].len() * 12],
        };

        let width = graph.width;
        let height = graph.height;

        for y in 0..height {
            for x in 0..width {
                let node = Node {
                    entry_cost: grid[y][x],
                };
                for _ in 0..12 {
                    /*
                     *  0 right once
                     *  1 right twice
                     *  2 right thrice
                     *  3 down once
                     *  4 down twice
                     *  5 down thrice
                     *  6 left once
                     *  7 left twice
                     *  8 left thrice
                     *  9 up once
                     * 10 up twice
                     * 11 up thrice
                     */
                    graph.add_node(node.clone());
                }
            }
        }

        for y in 0..height {
            for x in 0..width {
                if x < width - 1 {
                    for i in 0..12 {
                        if i == 0 || i == 1 {
                            graph.add_edge(
                                graph.get_index(x, y, i),
                                graph.get_index(x + 1, y, i + 1),
                            );
                            continue;
                        } else if i == 2 || (6..9).contains(&i) {
                            continue;
                        }
                        graph.add_edge(graph.get_index(x, y, i), graph.get_index(x + 1, y, 0));
                    }
                }
                if y < height - 1 {
                    for i in 0..12 {
                        if i == 3 || i == 4 {
                            graph.add_edge(
                                graph.get_index(x, y, i),
                                graph.get_index(x, y + 1, i + 1),
                            );
                            continue;
                        } else if i == 5 || (9..12).contains(&i) {
                            continue;
                        }
                        graph.add_edge(graph.get_index(x, y, i), graph.get_index(x, y + 1, 3));
                    }
                }
                if x > 0 {
                    for i in 0..12 {
                        if i == 6 || i == 7 {
                            graph.add_edge(
                                graph.get_index(x, y, i),
                                graph.get_index(x - 1, y, i + 1),
                            );
                            continue;
                        } else if i == 8 || (0..3).contains(&i) {
                            continue;
                        }
                        graph.add_edge(graph.get_index(x, y, i), graph.get_index(x - 1, y, 6));
                    }
                }
                if y > 0 {
                    for i in 0..12 {
                        if i == 9 || i == 10 {
                            graph.add_edge(
                                graph.get_index(x, y, i),
                                graph.get_index(x, y - 1, i + 1),
                            );
                            continue;
                        } else if i == 11 || (3..6).contains(&i) {
                            continue;
                        }
                        graph.add_edge(graph.get_index(x, y, i), graph.get_index(x, y - 1, 9));
                    }
                }
            }
        }

        graph
    }

    fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.edges[from].push(to);
    }

    fn get_index(&self, x: usize, y: usize, z: usize) -> usize {
        (y * self.width + x) * 12 + z
    }

    fn get_neighbors(&self, index: usize) -> Vec<usize> {
        self.edges.get(index).unwrap().clone()
    }

    fn get_cost(&self, index: usize) -> i32 {
        self.nodes[index].entry_cost
    }
}

fn walk(grid: &Vec<Vec<i32>>) -> i32 {
    let graph = Graph::new(grid);

    let mut costs = vec![std::i32::MAX; graph.nodes.len()];
    for i in 0..12 {
        costs[graph.get_index(0, 0, i)] = 0;
    }
    let mut neighbors: BinaryHeap<(Reverse<i32>, usize)> = BinaryHeap::new();
    for n in graph.get_neighbors(11) {
        neighbors.push((Reverse(graph.get_cost(n)), n));
    }

    let mut visited = vec![false; graph.nodes.len()];

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
            }
        }
    }

    costs[graph.width * graph.height * 12 - 12..]
        .iter()
        .min()
        .unwrap()
        .clone()
}

pub fn part1(path: &str) -> i32 {
    let input = std::fs::read_to_string(path).unwrap();
    let grid = input
        .split_ascii_whitespace()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    walk(&grid)
}
