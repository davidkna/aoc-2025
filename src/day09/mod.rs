use bstr::ByteSlice;
use itertools::Itertools;
use geo::{LineString, Polygon, Rect, coord, Contains};
use rayon::prelude::*;

pub const INPUT: &[u8] = include_bytes!("../../inputs/day09.txt");

fn parse_uint(s: &[u8]) -> u64 {
    s.iter().fold(0, |acc, &c| acc * 10 + (c - b'0') as u64)
}

pub fn part_1(input: &[u8]) -> u64 {
    let coords: Vec<(u64, u64)> = input
        .lines()
        .map(|line| {
            line.split_once_str(",")
                .map(|(x, y)| (parse_uint(x), parse_uint(y)))
                .unwrap()
        })
        .collect_vec();

    coords
        .iter()
        .tuple_combinations()
        .par_bridge()
        .map(|(&(x1, y1), &(x2, y2))| {
            let dx = x1.abs_diff(x2) + 1;
            let dy = y1.abs_diff(y2) + 1;

            (dx * dy) as u64
        })
        .max()
        .unwrap()
}


pub fn part_2(input: &[u8]) -> u64 {
    let coords: Vec<(u64, u64)> = input
        .lines()
        .map(|line| {
            line.split_once_str(",")
                .map(|(x, y)| (parse_uint(x), parse_uint(y)))
                .unwrap()
        })
        .collect_vec();

    let polygon: Polygon<f64> = Polygon::new(
        LineString::from(
            coords.iter()
                .map(|&(x, y)| coord! { x: x as f64, y: y as f64 })
                         .collect::<Vec<_>>()
        ),
        vec![],
    );

    coords
        .iter()
        .tuple_combinations()
        .par_bridge()
        .filter_map(|(&(x1, y1), &(x2, y2))| {
            let rect = Rect::new(
                coord! { x: x1 as f64, y: y1 as f64 },
                coord! { x: x2 as f64, y: y2 as f64 },
            );
            
            polygon.contains(&rect).then(|| {
                let dx = x1.abs_diff(x2) + 1;
                let dy = y1.abs_diff(y2) + 1;
                dx * dy
            })
        })
        .max()
        .unwrap()
}

pub fn run() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part_1() {
        let result = part_1(EXAMPLE);
        assert_eq!(result, 50);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(EXAMPLE);
        assert_eq!(result, 24);
    }
}
