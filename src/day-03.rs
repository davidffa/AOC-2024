use std::iter::{self, from_fn};

fn main() {
    let input = include_str!("../inputs/day-03.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[derive(Debug, PartialEq)]
enum Token {
    Mul,
    Do,
    Dont,
    Number(i32),
    Comma,
    Lparen,
    Rparen,
    Unknown,
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            'm' => {
                if chars.by_ref().take(2).collect::<String>() == "ul" {
                    tokens.push(Token::Mul);
                } else {
                    tokens.push(Token::Unknown);
                }
            }
            'd' => {
                let mut next_chars = chars.by_ref().take(3).collect::<String>();

                if next_chars == "o()" {
                    tokens.push(Token::Do);
                } else if next_chars == "on'" {
                    next_chars = chars.by_ref().take(3).collect::<String>();

                    if next_chars == "t()" {
                        tokens.push(Token::Dont);
                    } else {
                        tokens.push(Token::Unknown);
                    }
                } else {
                    tokens.push(Token::Unknown);
                }
            }
            '1'..='9' => {
                let n: i32 = iter::once(ch)
                    .chain(from_fn(|| chars.by_ref().next_if(|c| c.is_ascii_digit())))
                    .collect::<String>()
                    .parse()
                    .unwrap();

                tokens.push(Token::Number(n));
            }
            '(' => tokens.push(Token::Lparen),
            ')' => tokens.push(Token::Rparen),
            ',' => tokens.push(Token::Comma),
            _ => {
                if let Some(token) = tokens.last() {
                    if *token != Token::Unknown {
                        tokens.push(Token::Unknown);
                    }
                }
            }
        }
    }

    tokens
}

fn part1(input: &str) -> i32 {
    let tokens = tokenize(input);
    let mut ans = 0;

    // Not really proud of this part but it works :)
    for i in 0..tokens.len() - 5 {
        if tokens[i] == Token::Mul
            && tokens[i + 1] == Token::Lparen
            && tokens[i + 3] == Token::Comma
            && tokens[i + 5] == Token::Rparen
        {
            if let Token::Number(n1) = tokens[i + 2] {
                if let Token::Number(n2) = tokens[i + 4] {
                    ans += n1 * n2;
                }
            }
        }
    }

    ans
}

fn part2(input: &str) -> i32 {
    let tokens = tokenize(input);
    let mut ans = 0;
    let mut exec = true;

    for i in 0..tokens.len() - 5 {
        if tokens[i] == Token::Do {
            exec = true;
        } else if tokens[i] == Token::Dont {
            exec = false;
        }

        if exec
            && tokens[i] == Token::Mul
            && tokens[i + 1] == Token::Lparen
            && tokens[i + 3] == Token::Comma
            && tokens[i + 5] == Token::Rparen
        {
            if let Token::Number(n1) = tokens[i + 2] {
                if let Token::Number(n2) = tokens[i + 4] {
                    ans += n1 * n2;
                }
            }
        }
    }

    ans
}

#[allow(dead_code)]
const TEST_INPUT: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

#[allow(dead_code)]
const TEST_INPUT_P2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 161);
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT_P2), 48);
}
