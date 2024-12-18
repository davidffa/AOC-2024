use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

fn main() {
    let input = include_str!("../inputs/day-18.txt");

    println!("Part 1: {}", part1(input, 71, 1024));
    println!("Part 2: {:?}", part2(input, 71));
}

const DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    cost: u32,
    state: (usize, usize),
}

impl Node {
    fn new(state: (usize, usize), cost: u32) -> Self {
        Self { state, cost }
    }
}

fn dijkstra(grid_size: usize, obstacles: &HashSet<(usize, usize)>) -> u32 {
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    queue.push(Reverse(Node::new((0, 0), 0)));

    while let Some(Reverse(node)) = queue.pop() {
        if visited.contains(&node.state) {
            continue;
        }

        visited.insert(node.state);

        if node.state == (grid_size - 1, grid_size - 1) {
            return node.cost;
        }

        for (dx, dy) in DIRS {
            let nx = node.state.0 as i32 + dx;
            let ny = node.state.1 as i32 + dy;

            if 0 <= nx
                && nx < grid_size as i32
                && 0 <= ny
                && ny < grid_size as i32
                && !obstacles.contains(&(nx as usize, ny as usize))
            {
                queue.push(Reverse(Node::new(
                    (nx as usize, ny as usize),
                    node.cost + 1,
                )));
            }
        }
    }

    0
}

fn part1(input: &str, grid_size: usize, limit: usize) -> u32 {
    let mut obstacles = HashSet::new();
    let mut count = 0;

    input.lines().for_each(|line| {
        if count >= limit {
            return;
        }

        let mut splited = line.split(",");
        let x = splited.next().unwrap().parse::<usize>().unwrap();
        let y = splited.next().unwrap().parse::<usize>().unwrap();

        obstacles.insert((x, y));
        count += 1;
    });

    dijkstra(grid_size, &obstacles)
}

fn part2(input: &str, grid_size: usize) -> (usize, usize) {
    let mut all_obstacles = vec![];
    let mut count = 0;

    input.lines().for_each(|line| {
        let mut splited = line.split(",");
        let x = splited.next().unwrap().parse::<usize>().unwrap();
        let y = splited.next().unwrap().parse::<usize>().unwrap();

        all_obstacles.push((x, y));
        count += 1;
    });

    let mut lb = 0;
    let mut rb = all_obstacles.len();

    while lb < rb - 1 {
        let mb = (lb + rb) / 2;

        let mut obstacles = HashSet::new();

        for i in 0..mb {
            obstacles.insert(all_obstacles[i]);
        }

        let cost = dijkstra(grid_size, &obstacles);

        if cost == 0 {
            rb = mb;
        } else {
            lb = mb;
        }
    }

    all_obstacles[lb]
}

#[allow(dead_code)]
const TEST_INPUT: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT, 7, 12), 22);
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT, 7), (6, 1));
}
