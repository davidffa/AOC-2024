use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("../inputs/day-10.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

const DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];

fn dfs(map: &Vec<Vec<u32>>, pos: (i32, i32), dedup: bool) -> u32 {
    let mut stack = VecDeque::new();
    let mut visited = HashSet::new();
    stack.push_front(pos);

    let mut trailhead_count = 0;

    while let Some((x, y)) = stack.pop_front() {
        if dedup {
            if visited.contains(&(x, y)) {
                continue;
            }

            visited.insert((x, y));
        }

        if map[y as usize][x as usize] == 9 {
            trailhead_count += 1;
            continue;
        }

        for (dx, dy) in DIRS {
            let nx = x + dx;
            let ny = y + dy;

            if 0 <= nx && nx < map[0].len() as i32 && 0 <= ny && ny < map.len() as i32 {
                if map[ny as usize][nx as usize] == map[y as usize][x as usize] + 1 {
                    stack.push_front((nx, ny));
                }
            }
        }
    }

    trailhead_count
}

fn part1(input: &str) -> u32 {
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut ans = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 0 {
                ans += dfs(&map, (x as i32, y as i32), true);
            }
        }
    }

    ans
}

fn part2(input: &str) -> u32 {
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut ans = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 0 {
                ans += dfs(&map, (x as i32, y as i32), false);
            }
        }
    }

    ans
}

#[allow(dead_code)]
const TEST_INPUT: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 36);
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 81);
}
