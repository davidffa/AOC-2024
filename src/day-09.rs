fn main() {
    let input = include_str!("../inputs/day-09.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    let mut chars = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect::<Vec<_>>();
    let mut idx = 1;
    let mut file_idx = chars.len() - (chars.len() % 2 == 0) as usize - 1;
    let mut ans = 0u64;

    let mut pos = chars[0];

    while idx <= file_idx {
        if chars[idx] == 0 {
            idx += 1;
            continue;
        }

        if idx % 2 == 0 {
            let id = idx / 2;
            ans += id as u64 * chars[idx] * (pos + pos + chars[idx] - 1) / 2;
            pos += chars[idx];
            idx += 1;
            continue;
        }

        let id = file_idx / 2;
        if chars[file_idx] > chars[idx] {
            // Overflows
            ans += id as u64 * chars[idx] * (pos + pos + chars[idx] - 1) / 2;
            pos += chars[idx];
            chars[file_idx] -= chars[idx];

            idx += 1;
        } else {
            ans += id as u64 * chars[file_idx] * (pos + pos + chars[file_idx] - 1) / 2;
            pos += chars[file_idx];
            chars[idx] -= chars[file_idx];
            file_idx -= 2;
        }
    }

    ans
}

struct Block {
    id: u64,
    begin: u64,
    end: u64,
    consumed: bool,
}

impl Block {
    fn new(id: u64, begin: u64, end: u64) -> Self {
        Self {
            id,
            begin,
            end,
            consumed: false,
        }
    }

    fn size(&self) -> u64 {
        self.end + 1 - self.begin
    }
}

fn part2(input: &str) -> u64 {
    let mut blocks: Vec<Block> = Vec::new();
    let mut free: Vec<Block> = Vec::new();
    let mut moved_blocks: Vec<Block> = Vec::new();

    let mut id = 0u64;
    let mut pos = 0u64;
    for (i, ch) in input.trim().chars().enumerate() {
        let digit = ch.to_digit(10).unwrap() as u64;

        if digit == 0 {
            continue;
        }

        let block = Block::new(id, pos, pos + digit - 1);

        if i % 2 == 0 {
            blocks.push(block);
        } else {
            free.push(block);
        }

        pos += digit;

        if i % 2 == 0 {
            id += 1;
        }
    }

    for i in (0..blocks.len()).rev() {
        for j in 0..free.len() {
            let block = &mut blocks[i];
            let free = &mut free[j];

            if free.begin > block.end {
                break;
            }

            if block.size() <= free.size() {
                moved_blocks.push(Block::new(
                    block.id,
                    free.begin,
                    free.begin + block.size() - 1,
                ));
                // println!(
                //     "Moving block id {} to {} {} sz = {}",
                //     block.id,
                //     free.begin,
                //     free.begin + block.size() - 1,
                //     block.size()
                // );
                free.begin += block.size();
                block.end = block.begin;
                block.consumed = true;
                break;
            }
        }
    }

    blocks.extend(moved_blocks);

    blocks
        .iter()
        .filter(|b| !b.consumed)
        .map(|block| block.id * block.size() * (block.begin + block.end) / 2)
        .sum()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "\
2333133121414131402
";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 1928);
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 2858);
}
