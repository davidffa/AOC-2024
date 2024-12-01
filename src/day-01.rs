use std::collections::{BinaryHeap, HashMap};

fn main() {
    let input = include_str!("../inputs/day-01.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let mut heap1 = BinaryHeap::new();
    let mut heap2 = BinaryHeap::new();

    input.lines().for_each(|line| {
        let mut splitted = line.split_whitespace();

        heap1.push(splitted.next().unwrap().parse::<i32>().unwrap());
        heap2.push(splitted.next().unwrap().parse::<i32>().unwrap());
    });

    assert_eq!(heap1.len(), heap2.len());

    let mut ans = 0;

    while !heap1.is_empty() {
        ans += (heap1.pop().unwrap() - heap2.pop().unwrap()).abs()
    }

    ans
}

fn part2(input: &str) -> i32 {
    let mut first_list = Vec::new();
    let mut map = HashMap::new();

    input.lines().for_each(|line| {
        let mut splitted = line.split_whitespace();

        first_list.push(splitted.next().unwrap().parse().unwrap());
        let second = splitted.next().unwrap().parse::<i32>().unwrap();

        *map.entry(second).or_insert(0) += 1;
    });

    first_list
        .iter()
        .map(|el| el * map.get(&el).unwrap_or(&0))
        .sum()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 11);
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 31);
}
