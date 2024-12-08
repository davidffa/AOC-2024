use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../inputs/day-08.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[inline]
fn inside(x: i32, y: i32, w: usize, h: usize) -> bool {
    0 <= x && x < w as i32 && 0 <= y && y < h as i32
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn part1(input: &str) -> i32 {
    let mut map = parse_input(input);
    let mut antennas = HashMap::new();

    let w = map[0].len();
    let h = map.len();

    for y in 0..h {
        for x in 0..w {
            if map[y][x] != '.' {
                antennas
                    .entry(map[y][x])
                    .or_insert(HashSet::new())
                    .insert((x as i32, y as i32));
            }
        }
    }

    let mut ans = 0;

    antennas.values().for_each(|v| {
        for (x1, y1) in v {
            for (x2, y2) in v {
                if x1 == x2 && y1 == y2 {
                    continue;
                }

                let dx = x1 - x2;
                let dy = y1 - y2;

                if inside(x1 + dx, y1 + dy, w, h)
                    && map[(y1 + dy) as usize][(x1 + dx) as usize] != '#'
                {
                    ans += 1;
                    map[(y1 + dy) as usize][(x1 + dx) as usize] = '#';
                }

                if inside(x2 - dx, y2 - dy, w, h)
                    && map[(y2 - dy) as usize][(x2 - dx) as usize] != '#'
                {
                    ans += 1;
                    map[(y2 - dy) as usize][(x2 - dx) as usize] = '#';
                }
            }
        }
    });

    ans
}

fn part2(input: &str) -> i32 {
    let mut map = parse_input(input);
    let mut antennas = HashMap::new();

    let w = map[0].len();
    let h = map.len();

    for y in 0..h {
        for x in 0..w {
            if map[y][x] != '.' {
                antennas
                    .entry(map[y][x])
                    .or_insert(HashSet::new())
                    .insert((x as i32, y as i32));
            }
        }
    }

    let mut ans = 0;

    antennas.values().for_each(|v| {
        for (x1, y1) in v {
            for (x2, y2) in v {
                if x1 == x2 && y1 == y2 {
                    continue;
                }

                let dx = x1 - x2;
                let dy = y1 - y2;

                let mut nx = *x1;
                let mut ny = *y1;

                while inside(nx, ny, w, h) {
                    if map[ny as usize][nx as usize] != '#' {
                        ans += 1;
                        map[ny as usize][nx as usize] = '#';
                    }
                    nx += dx;
                    ny += dy;
                }

                nx = *x2;
                ny = *y2;

                while inside(nx, ny, w, h) {
                    if map[ny as usize][nx as usize] != '#' {
                        ans += 1;
                        map[ny as usize][nx as usize] = '#';
                    }
                    nx -= dx;
                    ny -= dy;
                }
            }
        }
    });

    ans
}

#[allow(dead_code)]
const TEST_INPUT: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

#[allow(dead_code)]
const TEST_INPUT2: &str = "\
..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........
";

#[allow(dead_code)]
const TEST_INPUT3: &str = "\
..........
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
..........
";

#[allow(dead_code)]
const TEST_INPUT4: &str = "\
T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........
";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 14);
}

#[test]
fn test_part1_2() {
    assert_eq!(part1(TEST_INPUT2), 2);
}

#[test]
fn test_part1_3() {
    assert_eq!(part1(TEST_INPUT3), 4);
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 34);
}

#[test]
fn test_part2_1() {
    assert_eq!(part2(TEST_INPUT4), 9);
}
