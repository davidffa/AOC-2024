use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("../inputs/day-04.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

const DIRS: [(i32, i32); 8] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, -1),
    (1, 1),
    (-1, 1),
    (1, -1),
];

#[derive(Clone)]
struct Node {
    pos: (i32, i32),
    dir: Option<(i32, i32)>,
    state: char,
    parent: Option<Box<Node>>,
}

impl Node {
    fn new(
        pos: (i32, i32),
        dir: Option<(i32, i32)>,
        state: char,
        parent: Option<Box<Node>>,
    ) -> Self {
        Self {
            pos,
            dir,
            state,
            parent,
        }
    }

    fn current_word(&self) -> String {
        let mut str = String::from(self.state);

        let mut node = self;

        while let Some(parent) = &node.parent {
            str.push(parent.state);
            node = parent;
        }

        str.chars().rev().collect()
    }
}

// I did a BFS, a bit overkill ig
fn part1(input: &str) -> i32 {
    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut open_nodes = VecDeque::new();
    let mut visited = HashSet::new();

    for y in 0..matrix.len() {
        for x in 0..matrix[0].len() {
            open_nodes.push_back(Node::new((x as i32, y as i32), None, matrix[y][x], None));
        }
    }

    let rows = matrix.len() as i32;
    let cols = matrix[0].len() as i32;

    let mut ans = 0;

    while let Some(node) = open_nodes.pop_front() {
        if visited.contains(&node.pos) {
            continue;
        }

        let (x, y) = node.pos;

        match node.current_word().as_str() {
            "X" => {
                visited.insert((x, y));

                for (dx, dy) in DIRS {
                    let x0 = x + dx;
                    let y0 = y + dy;
                    if 0 <= x0 && x0 < cols && 0 <= y0 && y0 < rows {
                        open_nodes.push_back(Node::new(
                            (x0, y0),
                            Some((dx, dy)),
                            matrix[y0 as usize][x0 as usize],
                            Some(Box::new(node.clone())),
                        ));
                    }
                }
            }
            "XM" | "XMA" => {
                for (dx, dy) in DIRS {
                    let x0 = x + dx;
                    let y0 = y + dy;
                    if 0 <= x0 && x0 < cols && 0 <= y0 && y0 < rows && node.dir.unwrap() == (dx, dy)
                    {
                        open_nodes.push_back(Node::new(
                            (x0, y0),
                            Some((dx, dy)),
                            matrix[y0 as usize][x0 as usize],
                            Some(Box::new(node.clone())),
                        ));
                    }
                }
            }
            "XMAS" => {
                ans += 1;

                let mut curr_node = &node;

                while let Some(parent) = &curr_node.parent {
                    curr_node = parent;
                }
            }
            _ => {}
        }
    }

    ans
}

const DIRS2: [((i32, i32), (i32, i32), (i32, i32), (i32, i32)); 4] = [
    ((-1, -1), (1, -1), (1, 1), (-1, 1)),
    ((-1, -1), (-1, 1), (1, 1), (1, -1)),
    ((1, 1), (-1, 1), (-1, -1), (1, -1)),
    ((1, 1), (1, -1), (-1, -1), (-1, 1)),
];

// Kinda ugly but idc
fn part2(input: &str) -> i32 {
    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut ans = 0;

    for y in 1..matrix.len() as i32 - 1 {
        for x in 1..matrix[0].len() as i32 - 1 {
            if matrix[y as usize][x as usize] == 'A' {
                for ((dxm1, dym1), (dxm2, dym2), (dxs1, dys1), (dxs2, dys2)) in DIRS2 {
                    let xm1 = (x + dxm1) as usize;
                    let ym1 = (y + dym1) as usize;
                    let xm2 = (x + dxm2) as usize;
                    let ym2 = (y + dym2) as usize;

                    let xs1 = (x + dxs1) as usize;
                    let ys1 = (y + dys1) as usize;
                    let xs2 = (x + dxs2) as usize;
                    let ys2 = (y + dys2) as usize;

                    if matrix[ym1][xm1] == 'M'
                        && matrix[ym2][xm2] == 'M'
                        && matrix[ys1][xs1] == 'S'
                        && matrix[ys2][xs2] == 'S'
                    {
                        ans += 1;
                        break;
                    }
                }
            }
        }
    }

    ans
}

#[allow(dead_code)]
const TEST_INPUT: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

const TEST_INPUT2: &str = "\
..X...
.SAMX.
.A..A.
XMAS.S
.X....
";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 18);
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 9);
}
