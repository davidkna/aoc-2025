use bstr::ByteSlice;
use itertools::Itertools;

pub const INPUT: &[u8] = include_bytes!("../../inputs/day04.txt");

pub fn part_1(input: &[u8]) -> usize {
    let map = input.lines().collect_vec();
    let mut total = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let c = map[y][x];
            if c != b'@' {
                continue;
            }
            let directions = [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ];
            let is_accessible = directions
                .iter()
                .filter(|(dy, dx)| {
                    let ny = y as isize + dy;
                    let nx = x as isize + dx;
                    if ny < 0 || ny >= map.len() as isize || nx < 0 || nx >= map[0].len() as isize {
                        return false;
                    }
                    let nc = map[ny as usize][nx as usize];
                    nc == b'@'
                })
                .count()
                < 4;
            if is_accessible {
                total += 1;
            }
        }
    }
    total
}

pub fn part_2(input: &[u8]) -> u64 {
    let mut map = input.lines().map(|line| line.to_vec()).collect_vec();
    let mut total_total = 0;

    loop {
        let mut total = 0;
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                let c = map[y][x];
                if c != b'@' {
                    continue;
                }
                let directions = [
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ];
                let is_accessible = directions
                    .iter()
                    .filter(|(dy, dx)| {
                        let ny = y as isize + dy;
                        let nx = x as isize + dx;
                        if ny < 0
                            || ny >= map.len() as isize
                            || nx < 0
                            || nx >= map[0].len() as isize
                        {
                            return false;
                        }
                        let nc = map[ny as usize][nx as usize];
                        nc == b'@'
                    })
                    .count()
                    < 4;
                if is_accessible {
                    total += 1;
                    map[y][x] = b'.';
                }
            }
        }
        if total == 0 {
            break;
        }
        total_total += total;
    }
    total_total
}

pub fn run() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part_1() {
        let result = part_1(EXAMPLE);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(EXAMPLE);
        assert_eq!(result, 43);
    }
}
