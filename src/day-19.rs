use std::collections::HashMap;

fn main() {
    let input = include_str!("../inputs/day-19.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn can_make(towels: &Vec<&str>, design: &str) -> bool {
    if design.is_empty() {
        return true;
    }

    let mut result = false;

    towels.iter().for_each(|t| {
        if design.starts_with(t) {
            result = result || can_make(towels, &design[t.len()..]);
        }
    });

    result
}

fn part1(input: &str) -> usize {
    let (towels, designs) = input.split_once("\n\n").unwrap();

    let towels = towels.trim().split(", ").collect::<Vec<_>>();

    designs
        .lines()
        .filter(|design| can_make(&towels, design))
        .count()
}

fn count_ways(dp: &mut HashMap<String, usize>, towels: &Vec<&str>, design: String) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(ways) = dp.get(&design) {
        return *ways;
    }

    let mut result = 0;

    towels.iter().for_each(|t| {
        if design.starts_with(t) {
            result = result + count_ways(dp, towels, design[t.len()..].to_string());
        }
    });

    dp.insert(design, result);
    result
}

fn part2(input: &str) -> usize {
    let (towels, designs) = input.split_once("\n\n").unwrap();

    let towels = towels.trim().split(", ").collect::<Vec<_>>();

    let mut dp = HashMap::new();

    designs
        .lines()
        .map(|design| count_ways(&mut dp, &towels, design.to_string()))
        .sum()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 6);
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 16);
}
