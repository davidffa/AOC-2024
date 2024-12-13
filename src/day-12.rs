use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!("../inputs/day-12.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn bfs(
    pos: (i32, i32),
    map: &Vec<Vec<char>>,
    visited: &mut HashSet<(i32, i32)>,
) -> (usize, HashMap<(i32, i32), HashSet<(i32, i32)>>) {
    let mut queue = VecDeque::new();
    queue.push_back(pos);

    let mut area = 0;
    let mut perimeter = HashMap::new();

    while let Some((x, y)) = queue.pop_front() {
        if visited.contains(&(x, y)) {
            continue;
        }

        visited.insert((x, y));

        area += 1;

        for (dx, dy) in DIRS {
            let nx = x + dx;
            let ny = y + dy;

            if 0 <= nx
                && nx < map[0].len() as i32
                && 0 <= ny
                && ny < map.len() as i32
                && map[ny as usize][nx as usize] == map[y as usize][x as usize]
            {
                queue.push_back((nx, ny));
            } else {
                perimeter
                    .entry((dx, dy))
                    .or_insert(HashSet::new())
                    .insert((nx, ny));
            }
        }
    }

    (area, perimeter)
}

fn part1(input: &str) -> usize {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut ans = 0;
    let mut visited = HashSet::new();

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let (area, perimeter) = bfs((x as i32, y as i32), &map, &mut visited);
            let perimeter: usize = perimeter.values().map(|v| v.len()).sum();

            ans += area * perimeter;
        }
    }

    ans
}

fn bfs_perimeter(perimeter: HashMap<(i32, i32), HashSet<(i32, i32)>>) -> usize {
    let mut sides = 0;

    for edges in perimeter.values() {
        let mut visited = HashSet::new();

        for pos in edges {
            if visited.contains(pos) {
                continue;
            }

            let mut queue = VecDeque::new();
            queue.push_back(*pos);

            sides += 1;

            while let Some((x, y)) = queue.pop_front() {
                if visited.contains(&(x, y)) {
                    continue;
                }
                visited.insert((x, y));

                for (dx, dy) in DIRS {
                    let nx = x + dx;
                    let ny = y + dy;

                    if edges.contains(&(nx, ny)) {
                        queue.push_back((nx, ny));
                    }
                }
            }
        }
    }

    sides
}

fn part2(input: &str) -> usize {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut ans = 0;
    let mut visited = HashSet::new();

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let (area, perimeter) = bfs((x as i32, y as i32), &map, &mut visited);

            let sides = bfs_perimeter(perimeter);

            ans += area * sides;
        }
    }

    ans
}

#[allow(dead_code)]
const TEST_INPUT: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 1930);
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 1206);
}
