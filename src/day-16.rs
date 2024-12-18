use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    rc::Rc,
};

fn main() {
    let input = include_str!("../inputs/day-16.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[derive(PartialEq, Eq)]
struct Node {
    score: u32,
    state: (usize, usize),
    dir: (i32, i32),
    parent: Option<Rc<Node>>,
}

impl Node {
    fn new(state: (usize, usize), dir: (i32, i32), score: u32, parent: Option<Rc<Node>>) -> Self {
        Self {
            state,
            dir,
            score,
            parent,
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

const DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn dijkstra(start: (usize, usize), map: &Vec<Vec<char>>, all_solutions: bool) -> u32 {
    let mut queue = BinaryHeap::new();
    let mut solutions: Vec<Rc<Node>> = Vec::new();
    let mut visited: HashMap<((i32, i32), (usize, usize)), u32> = HashMap::new();

    queue.push(Rc::new(Node::new(start, DIRS[0], 0, None)));

    while let Some(node) = queue.pop() {
        let (x, y) = node.state;

        if let Some(n) = visited.get(&(node.dir, node.state)) {
            if node.score > *n {
                continue;
            }
        }

        visited.insert((node.dir, node.state), node.score);

        if map[y][x] == 'E' {
            if !all_solutions {
                return node.score;
            }

            if let Some(sol) = solutions.first() {
                if node.score == sol.score {
                    solutions.push(node);
                }
            } else {
                solutions.push(node);
            }
            continue;
        }

        for (dx, dy) in DIRS {
            if (dx, dy) == (-node.dir.0, -node.dir.1) {
                continue;
            }

            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if 0 <= nx
                && nx < map[0].len() as i32
                && 0 <= ny
                && ny < map.len() as i32
                && map[ny as usize][nx as usize] != '#'
            {
                let changed_dir = (node.dir != (dx, dy)) as u32;
                queue.push(Rc::new(Node::new(
                    (nx as usize, ny as usize),
                    (dx, dy),
                    node.score + changed_dir * 1000 + 1,
                    Some(Rc::clone(&node)),
                )));
            }
        }
    }

    let mut paths = HashSet::new();

    for sol in solutions {
        let mut curr = Some(&sol);

        while let Some(node) = curr {
            paths.insert(node.state);
            curr = node.parent.as_ref();
        }
    }

    paths.len() as u32
}

fn part1(input: &str) -> u32 {
    let map = input
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 'S' {
                return dijkstra((x, y), &map, false);
            }
        }
    }

    unreachable!();
}

fn part2(input: &str) -> u32 {
    let map = input
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 'S' {
                return dijkstra((x, y), &map, true);
            }
        }
    }

    unreachable!();
}

#[allow(dead_code)]
const TEST_INPUT: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

#[allow(dead_code)]
const TEST_INPUT2: &str = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 7036);
}

#[test]
fn test_part1_2() {
    assert_eq!(part1(TEST_INPUT2), 11048);
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 45);
}

#[test]
fn test_part2_2() {
    assert_eq!(part2(TEST_INPUT2), 64);
}
