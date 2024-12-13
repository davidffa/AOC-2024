fn main() {
    let input = include_str!("../inputs/day-13.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|block| {
            let lines = block.lines();

            let tuples = lines
                .map(|line| {
                    let mut split = line.split(",");

                    if line.contains("=") {
                        let x = split.next().unwrap()[9..].parse::<u32>().unwrap();
                        let y = split.next().unwrap()[3..].parse::<u32>().unwrap();

                        (x, y)
                    } else {
                        let x = split.next().unwrap()[12..].parse::<u32>().unwrap();
                        let y = split.next().unwrap()[3..].parse::<u32>().unwrap();

                        (x, y)
                    }
                })
                .collect::<Vec<_>>();

            let (ax, ay) = tuples[0];
            let (bx, by) = tuples[1];
            let (px, py) = tuples[2];

            for a in 0..100 {
                for b in 0..100 {
                    if (a * ax + b * bx, a * ay + b * by) == (px, py) {
                        return 3 * a + b;
                    }
                }
            }

            0
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    input
        .split("\n\n")
        .map(|block| {
            let lines = block.lines();

            let tuples = lines
                .map(|line| {
                    let mut split = line.split(",");

                    if line.contains("=") {
                        let x = split.next().unwrap()[9..].parse::<i64>().unwrap();
                        let y = split.next().unwrap()[3..].parse::<i64>().unwrap();

                        (x + 10000000000000, y + 10000000000000)
                    } else {
                        let x = split.next().unwrap()[12..].parse::<i64>().unwrap();
                        let y = split.next().unwrap()[3..].parse::<i64>().unwrap();

                        (x, y)
                    }
                })
                .collect::<Vec<_>>();

            let (ax, ay) = tuples[0];
            let (bx, by) = tuples[1];
            let (px, py) = tuples[2];

            let det = ax * by - bx * ay;

            let a = (px * by - py * bx) as f64 / det as f64;
            let b = (ax * py - px * ay) as f64 / det as f64;

            if a.fract() != 0.0 || b.fract() != 0.0 {
                return 0;
            }

            (3.0 * a + b) as u64
        })
        .sum()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 480);
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 875318608908);
}
