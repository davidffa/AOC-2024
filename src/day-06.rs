fn main() {
    let input = include_str!("../inputs/day-06.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn next_dir(old_dir: (i32, i32)) -> (i32, i32) {
    match old_dir {
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        _ => unreachable!(),
    }
}

fn part1(input: &str) -> i32 {
    let mut map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut dir = (0, -1);
    let mut guard_x = 0;
    let mut guard_y = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '^' {
                guard_x = x as i32;
                guard_y = y as i32;
            }
        }
    }

    let mut ans = 1;

    while 0 <= guard_x
        && guard_x < map[0].len() as i32
        && 0 <= guard_y
        && guard_y < map.len() as i32
    {
        if map[guard_y as usize][guard_x as usize] == '.' {
            ans += 1;
            map[guard_y as usize][guard_x as usize] = 'X';
        } else if map[guard_y as usize][guard_x as usize] == '#' {
            guard_x -= dir.0;
            guard_y -= dir.1;
            dir = next_dir(dir);
        }

        let (dx, dy) = dir;

        guard_x += dx;
        guard_y += dy;
    }

    ans
}

fn dir_to_mask(dir: (i32, i32)) -> u8 {
    match dir {
        (0, -1) => 0b0001,
        (0, 1) => 0b0010,
        (1, 0) => 0b0100,
        (-1, 0) => 0b1000,
        _ => unreachable!(),
    }
}

fn has_loop(mut map: Vec<Vec<char>>, mut guard_x: i32, mut guard_y: i32) -> bool {
    let mut dir = (0, -1);
    let mut visited = vec![vec![0u8; map[0].len()]; map.len()];

    while 0 <= guard_x
        && guard_x < map[0].len() as i32
        && 0 <= guard_y
        && guard_y < map.len() as i32
    {
        let dir_mask = dir_to_mask(dir);

        if visited[guard_y as usize][guard_x as usize] & dir_mask == dir_mask {
            return true;
        }

        if map[guard_y as usize][guard_x as usize] == '.' {
            map[guard_y as usize][guard_x as usize] = 'X';
            visited[guard_y as usize][guard_x as usize] |= dir_mask;
        } else if map[guard_y as usize][guard_x as usize] == '#' {
            guard_x -= dir.0;
            guard_y -= dir.1;
            dir = next_dir(dir);
        }

        let (dx, dy) = dir;

        guard_x += dx;
        guard_y += dy;
    }

    false
}

fn part2(input: &str) -> i32 {
    let mut map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut guard_x = 0;
    let mut guard_y = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '^' {
                guard_x = x as i32;
                guard_y = y as i32;
            }
        }
    }

    let mut ans = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '.' {
                map[y][x] = '#';

                if has_loop(map.clone(), guard_x, guard_y) {
                    ans += 1;
                }

                map[y][x] = '.';
            }
        }
    }

    ans
}

#[allow(dead_code)]
const TEST_INPUT: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 41);
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 6);
}
