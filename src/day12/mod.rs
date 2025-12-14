use bstr::ByteSlice;

pub const INPUT: &[u8] = include_bytes!("../../inputs/day12.txt");

fn parse_uint(s: &[u8]) -> u16 {
    s.iter().fold(0, |acc, &c| acc * 10 + (c - b'0') as u16)
}

pub fn part_1(input: &[u8]) -> usize {
    let (_shapes_raw, regions_raw) = input.rsplit_once_str("\n\n").unwrap();

    regions_raw
        .lines()
        .filter(|region_block| {
            let (dimensions_raw, shape_counts_raw) = region_block.split_once_str(": ").unwrap();
            let (w, h) = dimensions_raw
                .split_once_str("x")
                .map(|(w, h)| (parse_uint(w), parse_uint(h)))
                .unwrap();
            let shape_count = shape_counts_raw.split_str(" ").map(parse_uint).sum::<u16>();

            w * h * 2 >= 17 * shape_count
        })
        .count()
}

pub fn run() {
    println!("Part 1: {}", part_1(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test]
    fn test_part_1() {
        let result = part_1(EXAMPLE);
        assert_eq!(result, 2);
    }
}
