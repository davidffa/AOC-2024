use std::{collections::HashMap, str::FromStr};

fn main() {
    let input = include_str!("../inputs/day-24.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[derive(PartialEq)]
enum Op {
    AND,
    OR,
    XOR,
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Op::AND),
            "OR" => Ok(Op::OR),
            "XOR" => Ok(Op::XOR),
            _ => Err(()),
        }
    }
}

struct Statement {
    lhs: String,
    op: Op,
    rhs: String,
    result: String,
}

fn parse_input(input: &str) -> (HashMap<String, u8>, Vec<Statement>) {
    let (initial, statements) = input.split_once("\n\n").unwrap();

    let mut initial_map = HashMap::new();

    initial.lines().for_each(|line| {
        let (var, val) = line.split_once(": ").unwrap();
        initial_map.insert(var.to_string(), val.parse::<u8>().unwrap());
    });

    let statements = statements
        .lines()
        .map(|stat| {
            let mut split = stat.split_whitespace();

            let lhs = split.next().unwrap().to_string();
            let op = Op::from_str(split.next().unwrap()).unwrap();
            let rhs = split.next().unwrap().to_string();
            split.next();
            let result = split.next().unwrap().to_string();

            Statement {
                lhs,
                op,
                rhs,
                result,
            }
        })
        .collect::<Vec<_>>();

    (initial_map, statements)
}

fn part1(input: &str) -> u64 {
    let (mut vars, mut statements) = parse_input(input);

    while !statements.is_empty() {
        let mut to_remove = Vec::new();

        for (i, stat) in statements.iter().enumerate() {
            if vars.contains_key(&stat.lhs) && vars.contains_key(&stat.rhs) {
                let lhs = vars[&stat.lhs];
                let rhs = vars[&stat.rhs];

                let res = match stat.op {
                    Op::AND => lhs & rhs,
                    Op::OR => lhs | rhs,
                    Op::XOR => lhs ^ rhs,
                };

                vars.insert(stat.result.clone(), res);
                to_remove.push(i);
            }
        }

        to_remove.iter().rev().for_each(|idx| {
            statements.remove(*idx);
        });
    }

    let mut z_vars = vars
        .iter()
        .filter(|v| v.0.starts_with("z"))
        .collect::<Vec<_>>();

    z_vars.sort_by(|a, b| a.0.cmp(b.0));

    let mut ans = 0;

    for (i, v) in z_vars.iter().enumerate() {
        ans |= (*v.1 as u64) << i;
    }

    ans
}

fn part2(input: &str) -> i32 {
    let mut ans = 0;

    ans
}

#[allow(dead_code)]
const TEST_INPUT: &str = "\
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 2024);
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 0);
}
