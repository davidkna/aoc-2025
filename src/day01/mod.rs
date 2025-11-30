pub const INPUT: &[u8] = include_bytes!("../../inputs/day01.txt");

pub fn part_1(input: &[u8]) -> usize {
    0
}

pub fn part_2(input: &[u8]) -> usize {
    0
}

pub fn run() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 0);
    }
}
