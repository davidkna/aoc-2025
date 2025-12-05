use bstr::ByteSlice;
use itertools::Itertools;

pub const INPUT: &[u8] = include_bytes!("../../inputs/day05.txt");

fn parse_uint(s: &[u8]) -> u64 {
    s.iter().fold(0, |acc, &c| acc * 10 + (c - b'0') as u64)
}

pub struct IntervalMap {
    intervals: Vec<(u64, u64)>,
}

impl IntervalMap {
    fn from_iter(iter: impl Iterator<Item = (u64, u64)>) -> Self {
        let intervals: Vec<(u64, u64)> = iter.sorted_unstable_by_key(|(start, _end)| *start).fold(
            Vec::new(),
            |mut acc, (start, end)| {
                if let Some((_last_start, last_end)) = acc.last_mut()
                    && *last_end >= start
                {
                    *last_end = (*last_end).max(end);
                    return acc;
                }
                acc.push((start, end));
                acc
            },
        );
        IntervalMap { intervals }
    }

    fn contains(&self, point: u64) -> bool {
        let result = self.intervals.binary_search_by(|&(start, end)| {
            if point < start {
                std::cmp::Ordering::Greater
            } else if point > end {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        });
        result.is_ok()
    }

    fn total_interval_length(&self) -> u64 {
        self.intervals
            .iter()
            .map(|(start, end)| end - start + 1)
            .sum()
    }
}

pub fn parse_input(input: &[u8]) -> IntervalMap {
    let intervals = input.lines().map(|line| {
        let (start, end) = line.split_once_str(b"-").unwrap();
        let start = parse_uint(start);
        let end = parse_uint(end);
        (start, end)
    });

    IntervalMap::from_iter(intervals)
}

pub fn part_1(input: &[u8]) -> usize {
    let (ranges, ingredients) = input.split_once_str("\n\n").unwrap();
    let imap = parse_input(ranges);

    ingredients
        .lines()
        .map(parse_uint)
        .filter(|&num| imap.contains(num))
        .count()
}

pub fn part_2(input: &[u8]) -> u64 {
    let (ranges, _ingredients) = input.split_once_str("\n\n").unwrap();
    let imap = parse_input(ranges);

    imap.total_interval_length()
}

pub fn run() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part_1() {
        let result = part_1(EXAMPLE);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(EXAMPLE);
        assert_eq!(result, 14);
    }
}
