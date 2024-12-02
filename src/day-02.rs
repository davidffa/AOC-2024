fn main() {
    let input = include_str!("../inputs/day-02.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn is_line_safe(elements: Vec<i32>) -> bool {
    let desc = elements[0] > elements[1];

    elements
        .iter()
        .zip(elements.iter().skip(1))
        .all(|(prev, curr)| {
            (prev - curr).abs() <= 3 && (desc && prev > curr || !desc && prev < curr)
        })
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|elements| is_line_safe(elements))
        .filter(|b| *b)
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|elements| {
            for i in 0..elements.len() {
                let mut elements2 = elements.clone();

                elements2.remove(i);

                if is_line_safe(elements2) {
                    return true;
                }
            }
            false
        })
        .filter(|b| *b)
        .count()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 2);
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 4);
}
