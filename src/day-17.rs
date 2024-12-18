fn main() {
    let input = include_str!("../inputs/day-17.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn combo(regs: &Vec<u32>, operand: u32) -> u32 {
    match operand {
        0..=3 => operand,
        4..=6 => regs[(operand - 4) as usize],
        _ => unreachable!(),
    }
}

fn part1(input: &str) -> String {
    let (regs, program) = input.split_once("\n\n").unwrap();
    let mut output = vec![];

    let mut regs = regs
        .lines()
        .map(|r| r[12..].parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let program = program
        .split_once(" ")
        .unwrap()
        .1
        .trim()
        .split(",")
        .map(|instruction| instruction.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut inst_ptr = program.iter().skip(0);

    while let (Some(opcode), Some(operand)) = (inst_ptr.next(), inst_ptr.next()) {
        match opcode {
            0 => regs[0] = regs[0] / (2u32.pow(combo(&regs, *operand))),
            1 => regs[1] = regs[1] ^ operand,
            2 => regs[1] = combo(&regs, *operand) % 8,
            3 => {
                if regs[0] != 0 {
                    inst_ptr = program.iter().skip(*operand as usize)
                }
            }
            4 => regs[1] = regs[1] ^ regs[2],
            5 => output.push((combo(&regs, *operand) % 8).to_string()),
            6 => regs[1] = regs[0] / (2u32.pow(combo(&regs, *operand))),
            7 => regs[2] = regs[0] / (2u32.pow(combo(&regs, *operand))),
            _ => unreachable!(),
        }
    }

    output.join(",")
}

fn part2(input: &str) -> i32 {
    let mut ans = 0;

    ans
}

#[allow(dead_code)]
const TEST_INPUT: &str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), "4,6,3,5,6,3,5,2,1,0");
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 0);
}
