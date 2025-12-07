use bstr::ByteSlice;

pub const INPUT: &[u8] = include_bytes!("../../inputs/day07.txt");

pub fn part_1(input: &[u8]) -> u64 {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let start = first_line.iter().position(|&c| c == b'S').unwrap();

    let mut beams = vec![false; first_line.len()];
    beams[start] = true;

    lines
        .fold((beams, 0u64), |(beams, mut splits), line| {
            let mut new_beams = beams.clone();

            for (i, &c) in line.iter().enumerate() {
                if c == b'.' {
                    continue;
                }
                if !beams[i] {
                    continue;
                }
                splits += 1;
                new_beams[i - 1] = true;
                new_beams[i + 1] = true;
                new_beams[i] = false;
            }

            (new_beams, splits)
        })
        .1
}

pub fn part_2(input: &[u8]) -> u64 {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let start = first_line.iter().position(|&c| c == b'S').unwrap();

    let mut beams = vec![0u64; first_line.len()];
    beams[start] = 1;

    lines
        .fold(beams, |beams: Vec<u64>, line| {
            let mut new_beams = beams.clone();

            for (i, &c) in line.iter().enumerate() {
                if c == b'.' {
                    continue;
                }
                if beams[i] == 0 {
                    continue;
                }
                new_beams[i - 1] += beams[i];
                new_beams[i + 1] += beams[i];
                new_beams[i] = 0;
            }
            new_beams
        })
        .into_iter()
        .sum()
}

pub fn run() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part_1() {
        let result = part_1(EXAMPLE);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(EXAMPLE);
        assert_eq!(result, 40);
    }
}
