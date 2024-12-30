use std::collections::HashMap;

fn main() {
    let input = include_str!("../inputs/day-22.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

const MOD: u64 = 16777216;

fn solve(mut n: u64, limit: u32) -> u64 {
    for _ in 0..limit {
        let mul = n * 64;
        n = (n ^ mul) % MOD;
        let div = n / 32;
        n = (n ^ div) % MOD;
        let mul2 = n * 2048;
        n = (n ^ mul2) % MOD;
    }

    n
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| solve(line.parse().unwrap(), 2000))
        .sum()
}

fn solve2(mut n: u64, limit: u32) -> HashMap<(i8, i8, i8, i8), u32> {
    let mut seq = Vec::new();
    let mut map = HashMap::new();
    let mut old_price = (n % 10) as u32;

    for _ in 0..limit {
        let mul = n * 64;
        n = (n ^ mul) % MOD;
        let div = n / 32;
        n = (n ^ div) % MOD;
        let mul2 = n * 2048;
        n = (n ^ mul2) % MOD;

        let price = (n % 10) as u32;

        seq.push((price as i32 - old_price as i32) as i8);
        old_price = price;

        if seq.len() >= 4 {
            let s = &seq[seq.len() - 4..];
            if !map.contains_key(&(s[0], s[1], s[2], s[3])) {
                map.insert((s[0], s[1], s[2], s[3]), price);
            }
        }
    }

    map
}

fn part2(input: &str) -> u32 {
    let mut map: HashMap<(i8, i8, i8, i8), u32> = HashMap::new();

    input.lines().for_each(|line| {
        let mp = solve2(line.parse().unwrap(), 2000);

        for (k, v) in mp {
            *map.entry(k).or_default() += v;
        }
    });

    *map.values().max().unwrap()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "\
1
10
100
2024
";

#[allow(dead_code)]
const TEST_INPUT2: &str = "\
1
2
3
2024
";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 37327623);
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT2), 23);
}
