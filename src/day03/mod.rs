use bstr::ByteSlice;

pub const INPUT: &[u8] = include_bytes!("../../inputs/day03.txt");

fn solve_line(line: &[u8], size: usize) -> u64 {
    let mut max_item = line[0];
    let mut max_item_idx = 0;

    for (i, &c) in line.iter().enumerate().take(line.len() - size + 1) {
        if c > max_item {
            max_item = c;
            max_item_idx = i;
        }
    };
    if size == 1 {
        return (max_item - b'0') as u64;
    }

    (max_item - b'0') as u64 * 10u64.pow((size - 1) as u32)
        + solve_line(&line[max_item_idx + 1..], size - 1)
}

pub fn part_1(input: &[u8]) -> u64 {
    input
        .lines()
        .map(|line| solve_line(line, 2))
        .sum()
}

pub fn part_2(input: &[u8]) -> u64 {
        input
        .lines()
        .map(|line| solve_line(line, 12))
        .sum()
}

pub fn run() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part_1() {
        let result = part_1(EXAMPLE);
        assert_eq!(result, 357);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(EXAMPLE);
        assert_eq!(result, 3121910778619);
    }
}
