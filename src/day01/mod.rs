use bstr::ByteSlice;

pub const INPUT: &[u8] = include_bytes!("../../inputs/day01.txt");

fn parse_uint(s: &[u8]) -> i32 {
    s.iter().fold(0, |acc, &c| acc * 10 + (c - b'0') as i32)
}

pub fn part_1(input: &[u8]) -> u32 {
    input
        .lines()
        .fold((50, 0), |(sum, zero_count), line| {
            let out = match line {
                [b'L', delta @ ..] => sum - parse_uint(delta),
                [b'R', delta @ ..] => sum + parse_uint(delta),
                _ => unreachable!(),
            }
            .rem_euclid(100);
            (out, zero_count + (out == 0) as i32)
        })
        .1 as u32
}

pub fn part_2(input: &[u8]) -> i32 {
    input
        .lines()
        .fold((50, 0), |(pos, past_crossings), line| {
            let dir = line[0];
            let dist: i32 = parse_uint(&line[1..]);

            if dist == 0 {
                return (pos, past_crossings);
            }

            let delta = match dir {
                b'L' => -dist,
                b'R' => dist,
                _ => unreachable!(),
            };

            let dist_to_zero = if delta < 0 {
                if pos == 0 { 100 } else { pos }
            } else {
                100 - pos
            };

            let new_crossings = 1 + (dist - dist_to_zero).div_floor(100);
            let new_pos = (pos + delta).rem_euclid(100);

            (new_pos, past_crossings + new_crossings)
        })
        .1
}

pub fn run() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 3);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 6);
    }
}
