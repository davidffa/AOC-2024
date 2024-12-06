use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../inputs/day-05.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn load_rules(rules: &str) -> HashMap<i32, HashSet<i32>> {
    let mut rules_map = HashMap::new();

    rules.lines().for_each(|line| {
        let (before, after) = line.split_once("|").unwrap();

        rules_map
            .entry(after.parse::<i32>().unwrap())
            .or_insert(HashSet::new())
            .insert(before.parse::<i32>().unwrap());
    });

    rules_map
}

fn part1(input: &str) -> i32 {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let rules_map = load_rules(rules);
    let mut ans = 0;

    updates.lines().for_each(|line| {
        let nums = line
            .split(",")
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let mut old_nums = HashSet::new();

        if nums.iter().all(|n| {
            old_nums.insert(n);

            if rules_map.get(&n).is_none() {
                return true;
            }

            rules_map
                .get(&n)
                .unwrap()
                .iter()
                .all(|rule_num| !nums.contains(rule_num) || old_nums.contains(rule_num))
        }) {
            ans += nums[nums.len() / 2];
        }
    });

    ans
}

fn part2(input: &str) -> i32 {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let rules_map = load_rules(rules);
    let mut ans = 0;

    updates.lines().for_each(|line| {
        let mut nums = line
            .split(",")
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let mut old_nums = HashSet::new();
        let mut is_valid = true;

        let mut i = 0;

        while i < nums.len() {
            let n = nums[i];

            if let Some(rule_nums) = rules_map.get(&n) {
                let is_num_valid = rule_nums
                    .iter()
                    .all(|rule_num| !nums.contains(rule_num) || old_nums.contains(rule_num));

                if !is_num_valid {
                    is_valid = false;

                    rule_nums.iter().for_each(|rule_num| {
                        if nums.contains(rule_num) && !old_nums.contains(rule_num) {
                            let idx = nums.iter().position(|x| x == rule_num).unwrap();
                            nums.remove(idx);
                            nums.insert(i, *rule_num);
                        }
                    });

                    old_nums.insert(nums[i]);
                    continue;
                }
            }

            old_nums.insert(nums[i]);

            i += 1;
        }

        if !is_valid {
            ans += nums[nums.len() / 2];
        }
    });

    ans
}

#[allow(dead_code)]
const TEST_INPUT: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 143);
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 123);
}
