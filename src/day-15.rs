use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("../inputs/day-15.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

const DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn free_space(
    map: &Vec<Vec<char>>,
    pos: (usize, usize),
    dir: (i32, i32),
) -> Option<(usize, usize)> {
    let mut nx = (pos.0 as i32 + dir.0) as usize;
    let mut ny = (pos.1 as i32 + dir.1) as usize;

    while map[ny][nx] != '.' && map[ny][nx] != '#' {
        nx = (nx as i32 + dir.0) as usize;
        ny = (ny as i32 + dir.1) as usize;
    }

    if map[ny][nx] == '.' {
        return Some((nx, ny));
    }

    None
}

fn execute_move(
    pos: &mut (usize, usize),
    map: &mut Vec<Vec<char>>,
    movement: char,
) -> (usize, usize) {
    let (dx, dy) = match movement {
        '^' => DIRS[1],
        '<' => DIRS[3],
        '>' => DIRS[2],
        'v' => DIRS[0],
        _ => unreachable!(),
    };

    if let Some((x, y)) = free_space(&map, *pos, (dx, dy)) {
        let (dx, dy) = (-dx, -dy);

        let mut px = x;
        let mut py = y;
        let mut nx = (x as i32 + dx) as usize;
        let mut ny = (y as i32 + dy) as usize;

        while map[py][px] != '@' {
            map[py][px] = map[ny][nx];
            px = nx;
            py = ny;
            nx = (nx as i32 + dx) as usize;
            ny = (ny as i32 + dy) as usize;
        }

        map[py][px] = '.';

        return ((px as i32 - dx) as usize, (py as i32 - dy) as usize);
    }

    *pos
}

fn part1(input: &str) -> usize {
    let mut splited_input = input.split("\n\n");
    let mut map = splited_input
        .next()
        .unwrap()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut pos = (0, 0);

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '@' {
                pos = (x, y);
                break;
            }
        }
    }

    splited_input.next().unwrap().chars().for_each(|movement| {
        if movement != '\n' {
            pos = execute_move(&mut pos, &mut map, movement)
        }
    });

    let mut ans = 0;

    for y in 1..map.len() - 1 {
        for x in 1..map[0].len() - 1 {
            if map[y][x] == 'O' {
                ans += 100 * y + x;
            }
        }
    }

    ans
}

fn can_move(
    pos: (usize, usize),
    map: &Vec<Vec<char>>,
    movement: char,
) -> Option<Vec<(usize, usize)>> {
    let mut queue = VecDeque::new();
    let mut visited = Vec::new();
    let dy = if movement == '^' { -1 } else { 1 };

    queue.push_back(pos);

    while let Some((x, y)) = queue.pop_front() {
        if visited.contains(&(x, y)) {
            continue;
        }

        visited.push((x, y));

        let ny = (y as i32 + dy) as usize;

        if map[ny][x] == '#' {
            return None;
        } else if map[ny][x] == '[' {
            queue.push_back((x, ny));
            queue.push_back((x + 1, ny));
        } else if map[ny as usize][x] == ']' {
            queue.push_back((x, ny));
            queue.push_back((x - 1, ny));
        }
    }

    Some(visited)
}

fn execute_move2(
    pos: &mut (usize, usize),
    map: &mut Vec<Vec<char>>,
    movement: char,
) -> (usize, usize) {
    let (dx, dy) = match movement {
        '^' => DIRS[1],
        '<' => DIRS[3],
        '>' => DIRS[2],
        'v' => DIRS[0],
        _ => unreachable!(),
    };

    if movement == '^' || movement == 'v' {
        let dy = if movement == '^' { -1 } else { 1 };
        if let Some(positions) = can_move(*pos, map, movement) {
            for (x, y) in positions.iter().rev() {
                map[(*y as i32 + dy) as usize][*x] = map[*y][*x];
                map[*y][*x] = '.';
            }

            pos.1 = (pos.1 as i32 + dy) as usize;
        }

        return *pos;
    }

    if let Some((x, y)) = free_space(&map, *pos, (dx, dy)) {
        let (dx, dy) = (-dx, -dy);

        let mut px = x;
        let mut py = y;
        let mut nx = (x as i32 + dx) as usize;
        let mut ny = (y as i32 + dy) as usize;

        while map[py][px] != '@' {
            map[py][px] = map[ny][nx];
            px = nx;
            py = ny;
            nx = (nx as i32 + dx) as usize;
            ny = (ny as i32 + dy) as usize;
        }

        map[py][px] = '.';

        return ((px as i32 - dx) as usize, (py as i32 - dy) as usize);
    }

    *pos
}

fn part2(input: &str) -> usize {
    let mut splited_input = input.split("\n\n");
    let map_str = splited_input
        .next()
        .unwrap()
        .replace(".", "..")
        .replace("#", "##")
        .replace("O", "[]")
        .replace("@", "@.");

    let mut map = map_str
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut pos = (0, 0);

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '@' {
                pos = (x, y);
                break;
            }
        }
    }

    splited_input.next().unwrap().chars().for_each(|movement| {
        if movement != '\n' {
            pos = execute_move2(&mut pos, &mut map, movement);
        }
    });

    let mut ans = 0;

    for y in 1..map.len() - 1 {
        for x in 1..map[0].len() - 1 {
            if map[y][x] == '[' {
                ans += 100 * y + x;
            }
        }
    }

    ans
}

#[allow(dead_code)]
const TEST_INPUT3: &str = "\
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
";

#[allow(dead_code)]
const TEST_INPUT2: &str = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

#[allow(dead_code)]
const TEST_INPUT: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 10092);
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 9021);
}
