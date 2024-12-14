use std::collections::{HashMap, HashSet, VecDeque};

const SECONDS: i32 = 100;

fn main() {
    let input = include_str!("../inputs/day-14.txt");

    println!("Part 1: {}", part1(input, 101, 103));
    println!("Part 2: {}", part2(input, 101, 103));
}

fn parse_input(input: &str) -> Vec<(i32, i32, i32, i32)> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(" ");
            let mut pos = split.next().unwrap().split(",");
            let px = pos.next().unwrap()[2..].parse().unwrap();
            let py = pos.next().unwrap().parse().unwrap();

            let mut v = split.next().unwrap().split(",");
            let vx = v.next().unwrap()[2..].parse().unwrap();
            let vy = v.next().unwrap().parse().unwrap();

            (px, py, vx, vy)
        })
        .collect::<Vec<_>>()
}

fn find_quadrant(x: i32, y: i32, w: i32, h: i32) -> Option<u8> {
    if x < w / 2 {
        if y < h / 2 {
            return Some(2u8);
        } else if y > h / 2 {
            return Some(3u8);
        }
    } else if x > w / 2 {
        if y < h / 2 {
            return Some(1u8);
        } else if y > h / 2 {
            return Some(4u8);
        }
    }

    None
}

fn part1(input: &str, map_width: i32, map_height: i32) -> u32 {
    let robots = parse_input(input);

    let mut quads = HashMap::with_capacity(4);

    robots.iter().for_each(|(x, y, dx, dy)| {
        let nx = (*x + SECONDS * *dx).rem_euclid(map_width);
        let ny = (*y + SECONDS * *dy).rem_euclid(map_height);

        let quadrant = find_quadrant(nx, ny, map_width, map_height);

        if let Some(quadrant) = quadrant {
            *quads.entry(quadrant).or_insert(0) += 1;
        }
    });

    quads.values().fold(1, |acc, v| acc * v)
}

const DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn are_all_together(robots_map: &Vec<Vec<bool>>, map_width: i32, map_height: i32) -> bool {
    let mut stack = VecDeque::new();
    let mut vis = HashSet::new();
    let threshold = robots_map
        .iter()
        .flat_map(|r| r.iter())
        .filter(|&&v| v)
        .count()
        / 3;

    for y in 0..map_height {
        for x in 0..map_width {
            if !robots_map[y as usize][x as usize] {
                continue;
            }

            stack.clear();
            let mut count = 0;

            stack.push_front((x, y));

            while let Some((x, y)) = stack.pop_front() {
                if vis.contains(&(x, y)) {
                    continue;
                }

                vis.insert((x, y));
                count += 1;

                for (dx, dy) in DIRS {
                    let nx = x + dx;
                    let ny = y + dy;

                    if 0 <= nx
                        && nx < map_width
                        && 0 <= ny
                        && ny < map_height
                        && robots_map[ny as usize][nx as usize]
                    {
                        stack.push_front((nx, ny));
                    }
                }
            }

            if count > threshold {
                return true;
            }
        }
    }

    false
}

fn part2(input: &str, map_width: i32, map_height: i32) -> u32 {
    let mut robots = parse_input(input);

    let mut ans = 0;

    loop {
        robots.iter_mut().for_each(|(x, y, dx, dy)| {
            *x = (*x + *dx).rem_euclid(map_width);
            *y = (*y + *dy).rem_euclid(map_height);
        });

        let mut robots_map = vec![vec![false; map_width as usize]; map_height as usize];

        robots.iter().for_each(|(x, y, _, _)| {
            robots_map[*y as usize][*x as usize] = true;
        });

        ans += 1;

        if are_all_together(&robots_map, map_width, map_height) {
            for y in 0..map_height {
                for x in 0..map_width {
                    print!("{}", robots_map[y as usize][x as usize] as u8);
                }
                println!();
            }
            break;
        }
    }

    ans
}

#[allow(dead_code)]
const TEST_INPUT: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT, 11, 7), 12);
}
