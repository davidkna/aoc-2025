use bstr::ByteSlice;
use rayon::prelude::*;

pub const INPUT: &[u8] = include_bytes!("../../inputs/day02.txt");

fn parse_uint(s: &[u8]) -> u64 {
    s.iter().fold(0, |acc, &c| acc * 10 + (c - b'0') as u64)
}

fn is_invalid_part1(s: &[u8]) -> bool {
    let len = s.len();
    if !len.is_multiple_of(2) {
        return false;
    }
    let half = len / 2;
    s[..half] == s[half..]
}

fn is_invalid_part2(s: &[u8]) -> bool {
    let len = s.len();

    // Check if it's made of a static pattern
    (1..=(len / 2)).any(|pattern_len| {
        if !len.is_multiple_of(pattern_len) {
            return false;
        }

        let (pattern, remainder) = s.split_at(pattern_len);
        remainder.chunks(pattern_len).all(|chunk| chunk == pattern)
    })
}

fn increment(s: &mut Vec<u8>) {
    let mut i = s.len();
    while i > 0 {
        i -= 1;
        if s[i] < b'9' {
            s[i] += 1;
            return;
        }
        s[i] = b'0';
    }
    // Overflow:
    s.insert(0, b'1');
}

fn compare(a: &[u8], b: &[u8]) -> std::cmp::Ordering {
    match a.len().cmp(&b.len()) {
        std::cmp::Ordering::Equal => a.cmp(b),
        other => other,
    }
}

pub fn solve(input: &[u8], part2: bool) -> u64 {
    input
        .split_str(b",")
        .par_bridge()
        .map(|range| {
            let (start, end) = range.split_once_str(b"-").unwrap();

            let mut current = start.to_vec();
            let mut total = 0;

            while compare(&current, end) <= std::cmp::Ordering::Equal {
                let invalid = if part2 {
                    is_invalid_part2(&current)
                } else {
                    is_invalid_part1(&current)
                };

                if invalid {
                    total += parse_uint(&current);
                }
                increment(&mut current);
            }
            total
        })
        .sum()
}

pub fn part_1(input: &[u8]) -> u64 {
    solve(input, false)
}

pub fn part_2(input: &[u8]) -> u64 {
    solve(input, true)
}

pub fn run() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part_1() {
        let result = part_1(EXAMPLE);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(EXAMPLE);
        assert_eq!(result, 4174379265);
    }
}
