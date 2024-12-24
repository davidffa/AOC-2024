use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!("../inputs/day-20.txt");

    println!("Part 1: {}", part1(input, 100));
    println!("Part 2: {}", part2(input, 100));
}

const DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn dfs(map: &Vec<Vec<char>>, pos: (usize, usize)) -> HashMap<(usize, usize), u32> {
    let mut stack = VecDeque::new();
    let mut path = HashMap::new();

    stack.push_front((pos.0, pos.1, 0));

    while let Some((x, y, cost)) = stack.pop_front() {
        if path.contains_key(&(x, y)) {
            continue;
        }

        path.insert((x, y), cost);

        if map[y as usize][x as usize] == 'E' {
            return path;
        }

        for (dx, dy) in DIRS {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if inside(&map, (nx, ny)) && map[ny as usize][nx as usize] != '#' {
                stack.push_front((nx as usize, ny as usize, cost + 1));
            }
        }
    }

    unreachable!()
}

#[inline]
fn inside(map: &Vec<Vec<char>>, pos: (i32, i32)) -> bool {
    let (x, y) = pos;
    0 <= x && x < map[0].len() as i32 && 0 <= y && y < map.len() as i32
}

#[derive(PartialEq, Eq)]
struct Node {
    state: (usize, usize),
    depth: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.depth.cmp(&self.depth)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Node {
    fn new(state: (usize, usize), depth: u32) -> Self {
        Self { state, depth }
    }
}

fn cheat(
    map: &Vec<Vec<char>>,
    path: &HashMap<(usize, usize), u32>,
    min: u32,
    cheat_len: u32,
) -> u32 {
    let mut ans = 0;
    let mut seen_cheats = HashSet::new();

    for ((x, y), cost) in path {
        let mut heap = BinaryHeap::new();
        let mut visited = HashSet::new();

        heap.push(Node::new((*x, *y), 0));

        while let Some(node) = heap.pop() {
            let (ex, ey) = node.state;
            let depth = node.depth;
            if visited.contains(&(ex, ey)) {
                continue;
            }
            visited.insert((ex, ey));

            if let Some(cost2) = path.get(&(ex, ey)) {
                if cost + depth < *cost2 && cost2 - cost - depth >= min {
                    if !seen_cheats.contains(&(x, y, ex, ey)) {
                        ans += 1;
                        seen_cheats.insert((x, y, ex, ey));
                    }
                }
            }

            if depth < cheat_len {
                for (dx, dy) in DIRS {
                    let nx = ex as i32 + dx;
                    let ny = ey as i32 + dy;

                    if inside(&map, (nx, ny)) {
                        heap.push(Node::new((nx as usize, ny as usize), depth + 1));
                    }
                }
            }
        }
    }
    ans
}

fn part1(input: &str, min: u32) -> u32 {
    let map = input
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 'S' {
                let path = dfs(&map, (x, y));
                return cheat(&map, &path, min, 2);
            }
        }
    }

    unreachable!()
}

fn part2(input: &str, min: u32) -> u32 {
    let map = input
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 'S' {
                let path = dfs(&map, (x, y));
                return cheat(&map, &path, min, 20);
            }
        }
    }

    unreachable!()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT, 0), 44);
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT, 50), 285);
}
