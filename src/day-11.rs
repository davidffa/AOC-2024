use std::collections::HashMap;

fn main() {
    let input = include_str!("../inputs/day-11.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn blink(stones: &HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut new_stones: HashMap<u64, usize> = HashMap::new();

    for (stone, amount) in stones.iter() {
        let stone_str = stone.to_string();

        if *stone == 0 {
            *new_stones.entry(1).or_default() += amount;
        } else if stone_str.len() % 2 == 0 {
            *new_stones
                .entry(stone_str[..stone_str.len() / 2].parse().unwrap())
                .or_default() += amount;
            *new_stones
                .entry(stone_str[stone_str.len() / 2..].parse().unwrap())
                .or_default() += amount;
        } else {
            *new_stones.entry(*stone * 2024).or_default() += amount;
        }
    }

    new_stones
}

fn part1(input: &str) -> usize {
    let mut stones = HashMap::new();

    input.split_whitespace().for_each(|stone| {
        let stone = stone.parse::<u64>().unwrap();

        *stones.entry(stone).or_default() += 1;
    });

    for _ in 0..25 {
        stones = blink(&stones);
    }

    stones.values().sum()
}

fn part2(input: &str) -> usize {
    let mut stones = HashMap::new();

    input.split_whitespace().for_each(|stone| {
        let stone = stone.parse::<u64>().unwrap();

        *stones.entry(stone).or_default() += 1;
    });

    for _ in 0..75 {
        stones = blink(&stones);
    }

    stones.values().sum()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "\
125 17
";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 55312);
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 65601038650482);
}
