fn main() {
    let input = include_str!("../inputs/day-07.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn is_valid(result: u64, nums: &[u64], acc: u64) -> bool {
    if acc > result {
        return false;
    }

    if nums.is_empty() {
        return acc == result;
    }

    return is_valid(result, &nums[1..], acc + nums[0])
        | is_valid(result, &nums[1..], acc * nums[0]);
}

fn is_valid_part2(result: u64, nums: &[u64], acc: u64) -> bool {
    if acc > result {
        return false;
    }

    if nums.is_empty() {
        return acc == result;
    }

    return is_valid_part2(result, &nums[1..], acc + nums[0])
        | is_valid_part2(result, &nums[1..], acc * nums[0])
        | is_valid_part2(
            result,
            &nums[1..],
            acc * 10u64.pow(nums[0].ilog10() + 1) + nums[0],
        );
}

fn solve(line: &str) -> u64 {
    let (result, nums) = line.split_once(":").expect("To input be well formed");

    let result = result.parse::<u64>().unwrap();
    let nums = nums
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    if is_valid(result, &nums[1..], nums[0]) {
        return result;
    }

    0
}

fn solve_p2(line: &str) -> u64 {
    let (result, nums) = line.split_once(":").expect("To input be well formed");

    let result = result.parse::<u64>().unwrap();
    let nums = nums
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    if is_valid_part2(result, &nums[1..], nums[0]) {
        return result;
    }

    0
}

fn part1(input: &str) -> u64 {
    input.lines().map(|line| solve(line)).sum()
}

fn part2(input: &str) -> u64 {
    input.lines().map(|line| solve_p2(line)).sum()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 3749);
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 11387);
}
