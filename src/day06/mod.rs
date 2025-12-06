use bstr::ByteSlice;
use itertools::Itertools;

pub const INPUT: &[u8] = include_bytes!("../../inputs/day06.txt");

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Operation {
    Add,
    Multiply,
}

pub fn part_1(input: &[u8]) -> u64 {
    let (numbers, operations) = input.rsplit_once_str("\n").unwrap();

    let numbers = numbers
        .lines()
        .map(|line| {
            let (mut nums, last_num) =
                line.iter()
                    .fold((Vec::new(), None), |(mut nums, mut current), &c| {
                        match c {
                            b' ' if current.is_some() => {
                                nums.push(current.take().unwrap());
                            }
                            b' ' => {}
                            b'0'..=b'9' => {
                                current = Some(current.unwrap_or(0) * 10 + (c - b'0') as u64);
                            }
                            _ => unreachable!(),
                        }
                        (nums, current)
                    });
            if let Some(num) = last_num {
                nums.push(num);
            }
            nums
        })
        .collect_vec();

    let operations: Vec<Operation> = operations
        .iter()
        .filter_map(|&c| match c {
            b'+' => Some(Operation::Add),
            b'*' => Some(Operation::Multiply),
            _ => None,
        })
        .collect();

    let mut result = 0;
    for (col, op) in operations.iter().enumerate() {
        let col_result: u64 = if op == &Operation::Add {
            numbers.iter().map(|nums| nums[col]).sum()
        } else {
            numbers.iter().map(|nums| nums[col]).product()
        };
        result += col_result;
    }

    result
}

pub fn part_2(input: &[u8]) -> u64 {
    let (numbers, operations) = input.rsplit_once_str("\n").unwrap();

    let numbers = numbers.lines().collect_vec();

    let operations: Vec<Operation> = operations
        .iter()
        .filter_map(|&c| match c {
            b'+' => Some(Operation::Add),
            b'*' => Some(Operation::Multiply),
            _ => None,
        })
        .collect();

    let mut result = 0;
    let mut op_iter = operations.iter();
    let mut op = None;
    let mut col_result = 0;
    for col in 0..numbers[0].len() {
        if col == 0 || numbers.iter().all(|nums| nums[col] == b' ') {
            result += col_result;
            op = op_iter.next();
            col_result = match op {
                Some(Operation::Add) => 0,
                Some(Operation::Multiply) => 1,
                None => 0,
            };
            if col != 0 {
                continue;
            }
        }

        let col_num = numbers
            .iter()
            .map(|nums| nums[col])
            .filter(|&n| n != b' ')
            .fold(0, |acc, n| acc * 10 + (n - b'0') as u64);

        match op {
            Some(Operation::Add) => col_result += col_num,
            Some(Operation::Multiply) => col_result *= col_num,
            None => unreachable!(),
        }
    }

    result + col_result
}

pub fn run() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = concat_bytes!(
        b"123 328  51 64 \n",
        b" 45 64  387 23 \n",
        b"  6 98  215 314\n",
        b"*   +   *   +  "
    );

    #[test]
    fn test_part_1() {
        let result = part_1(EXAMPLE);
        assert_eq!(result, 4277556);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(EXAMPLE);
        assert_eq!(result, 3263827);
    }
}
