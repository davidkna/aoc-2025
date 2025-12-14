use std::collections::VecDeque;

use bitvec::prelude::*;
use bstr::ByteSlice;
use itertools::Itertools;
use rayon::prelude::*;

pub const INPUT: &[u8] = include_bytes!("../../inputs/day10.txt");

fn parse_uint(s: &[u8]) -> u32 {
    s.iter().fold(0, |acc, &c| acc * 10 + (c - b'0') as u32)
}

pub fn part_1(input: &[u8]) -> u32 {
    input
        .lines()
        .par_bridge()
        .map(|line| {
            let (target, rest) = line.split_once_str(" ").unwrap();
            let target = &target[1..target.len() - 1]; // remove [ ]
            let target = target.iter().fold(
                BitVec::<u8, Msb0>::with_capacity(target.len()),
                |mut bv, &c| {
                    bv.push(c == b'#');
                    bv
                },
            );

            let (buttons, _joltage) = rest.rsplit_once_str(" ").unwrap();
            let buttons = buttons.split_str(" ")
            .map(|x| &x[1..x.len()-1]) // remove ( )
            .map(|s| s.split_str(",").fold(bitvec![u8, Msb0; 0; target.len()], |mut bv, idx_bytes| {
                let idx = parse_uint(idx_bytes) as usize;
                bv.set(idx, true);
                bv
            }))
            .collect_vec();

            let current: BitVec<u8, Msb0> = bitvec![u8, Msb0; 0; target.len()];
            let mut q: VecDeque<BitVec<u8, Msb0>> = VecDeque::new();
            q.push_back(current);
            let mut steps = 0u32;
            while !q.is_empty() {
                let level_size = q.len();
                let mut level_seen = fnv::FnvHashSet::default();
                for _ in 0..level_size {
                    let state = q.pop_front().unwrap();
                    for button in &buttons {
                        let mut next_state = state.clone();
                        next_state ^= button;
                        if next_state == target {
                            return steps + 1;
                        }
                        if level_seen.insert(next_state.clone()) {
                            q.push_back(next_state);
                        }
                    }
                }
                steps += 1;
            }
            unreachable!()
        })
        .sum()
}

pub fn part_2(input: &[u8]) -> u64 {
    input
        .lines()
        .par_bridge()
        .map(|line| {
            let (_indicators, rest) = line.split_once_str(" ").unwrap();
            let (buttons, joltage) = rest.rsplit_once_str(" ").unwrap();
            let joltage = &joltage[1..joltage.len() - 1];
            let target: Vec<u64> = joltage
                .split_str(",")
                .map(|x| parse_uint(x) as u64)
                .collect_vec();

            let buttons: Vec<Vec<u64>> = buttons
                .split_str(" ")
                .map(|x| &x[1..x.len() - 1])
                .map(|s| {
                    s.split_str(",")
                        .fold(vec![0u64; target.len()], |mut v, idx_bytes| {
                            let idx = parse_uint(idx_bytes) as usize;
                            v[idx] = 1;
                            v
                        })
                })
                .collect_vec();

            let opt = z3::Optimize::new();

            let vars: Vec<z3::ast::Int> = buttons
                .iter()
                .enumerate()
                .map(|(i, _)| z3::ast::Int::new_const(format!("x_{}", i)))
                .collect_vec();

            for var in &vars {
                opt.assert(&var.ge(0));
            }

            for (idx, &tgt) in target.iter().enumerate() {
                let sum_expr = buttons
                    .iter()
                    .enumerate()
                    .map(|(i, button)| {
                        let coeff = button[idx] as i64;
                        z3::ast::Int::from_i64(coeff) * &vars[i]
                    })
                    .sum::<z3::ast::Int>();
                opt.assert(&sum_expr.eq(z3::ast::Int::from_i64(tgt as i64)));
            }

            let total_presses = vars.iter().sum::<z3::ast::Int>();
            opt.minimize(&total_presses);

            opt.check(&[]);
            opt.get_model()
                .unwrap()
                .eval(&total_presses, true)
                .unwrap()
                .as_i64()
                .unwrap() as u64
        })
        .sum()
}

pub fn run() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_part_1() {
        let result = part_1(EXAMPLE);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(EXAMPLE);
        assert_eq!(result, 33);
    }
}
