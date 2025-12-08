use bstr::ByteSlice;
use itertools::Itertools;

pub const INPUT: &[u8] = include_bytes!("../../inputs/day08.txt");

fn parse_uint(s: &[u8]) -> u64 {
    s.iter().fold(0, |acc, &c| acc * 10 + (c - b'0') as u64)
}

fn euclidean_distance(a: (u64, u64, u64), b: (u64, u64, u64)) -> u64 {
    let dx = (a.0 as i64 - b.0 as i64).pow(2);
    let dy = (a.1 as i64 - b.1 as i64).pow(2);
    let dz = (a.2 as i64 - b.2 as i64).pow(2);
    // skip sqrt for performance reasons
    (dx + dy + dz) as u64
}

pub fn part_1(input: &[u8], steps: usize) -> usize {
    let coords: Vec<(u64, u64, u64)> = input
        .lines()
        .map(|line| {
            line.splitn_str(3, ",")
                .collect_array()
                .map(|[x, y, z]| (parse_uint(x), parse_uint(y), parse_uint(z)))
                .unwrap()
        })
        .collect_vec();

    let n = coords.len();
    let mut dist = vec![vec![u64::MAX; n]; n];

    for i in 0..n {
        dist[i][i] = 0;
        for j in (i + 1)..n {
            let d = euclidean_distance(coords[i], coords[j]);
            dist[i][j] = d;
            dist[j][i] = d;
        }
    }

    let mut junction: Vec<Option<usize>> = vec![None; n];
    let mut next_junction_id = 0;
    for _ in 0..steps {
        // Find the closest pair of nodes
        let mut min_dist = u64::MAX;
        let mut min_i = 0;
        let mut min_j = 0;
        for i in 0..n {
            for j in (i + 1)..n {
                if dist[i][j] < min_dist {
                    min_dist = dist[i][j];
                    min_i = i;
                    min_j = j;
                }
            }
        }

        // Create a new junction
        let junction_id = match (junction[min_i], junction[min_j]) {
            (Some(id), Some(id2)) => {
                let new_id = id.min(id2);
                for j in junction.iter_mut() {
                    if *j == Some(id) || *j == Some(id2) {
                        *j = Some(new_id);
                    }
                }
                new_id
            }
            (Some(id), None) | (None, Some(id)) => id,
            (None, None) => {
                let id = next_junction_id;
                next_junction_id += 1;
                id
            }
        };

        junction[min_i] = Some(junction_id);
        junction[min_j] = Some(junction_id);

        dist[min_i][min_j] = u64::MAX;
        dist[min_j][min_i] = u64::MAX;
    }

    // Three largest junctions
    junction
        .into_iter()
        .fold(std::collections::HashMap::new(), |mut acc, j| {
            if let Some(j) = j {
                *acc.entry(j).or_insert(0) += 1;
            }
            acc
        })
        .into_values()
        .sorted_unstable_by(|a, b| b.cmp(a))
        .take(3)
        .product()
}

pub fn part_2(input: &[u8]) -> u64 {
    let coords: Vec<(u64, u64, u64)> = input
        .lines()
        .map(|line| {
            line.splitn_str(3, ",")
                .collect_array()
                .map(|[x, y, z]| (parse_uint(x), parse_uint(y), parse_uint(z)))
                .unwrap()
        })
        .collect_vec();

    let n = coords.len();
    let mut dist = vec![vec![u64::MAX; n]; n];

    for i in 0..n {
        dist[i][i] = 0;
        for j in (i + 1)..n {
            let d = euclidean_distance(coords[i], coords[j]);
            dist[i][j] = d;
            dist[j][i] = d;
        }
    }

    let mut junction: Vec<Option<usize>> = vec![None; n];
    let mut next_junction_id = 0;
    loop {
        // Find the closest pair of nodes
        let mut min_dist = u64::MAX;
        let mut min_i = 0;
        let mut min_j = 0;
        for i in 0..n {
            for j in (i + 1)..n {
                if dist[i][j] < min_dist {
                    min_dist = dist[i][j];
                    min_i = i;
                    min_j = j;
                }
            }
        }

        let j1 = junction[min_i].or(junction[min_j]);
        let j2 = junction[min_j].or(junction[min_i]);
        if j1.is_some()
            && junction
                .iter()
                .enumerate()
                .filter(|&(idx, _)| idx != min_i && idx != min_j)
                .all(|(_, j)| *j == j1 || *j == j2)
        {
            return coords[min_i].0 * coords[min_j].0;
        }

        // Create a new junction
        let junction_id = match (junction[min_i], junction[min_j]) {
            (Some(id), Some(id2)) => {
                let new_id = id.min(id2);
                for j in junction.iter_mut() {
                    if *j == Some(id) || *j == Some(id2) {
                        *j = Some(new_id);
                    }
                }
                new_id
            }
            (Some(id), None) | (None, Some(id)) => id,
            (None, None) => {
                let id = next_junction_id;
                next_junction_id += 1;
                id
            }
        };

        junction[min_i] = Some(junction_id);
        junction[min_j] = Some(junction_id);
        // Update distances
        for k in 0..n {
            let j_id = junction[k];
            if j_id != Some(junction_id) {
                continue;
            }
            dist[min_i][k] = u64::MAX;
            dist[k][min_i] = u64::MAX;
            dist[min_j][k] = u64::MAX;
            dist[k][min_j] = u64::MAX;
        }
    }
}

pub fn run() {
    println!("Part 1: {}", part_1(INPUT, 1000));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_part_1() {
        let result = part_1(EXAMPLE, 10);
        assert_eq!(result, 40);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(EXAMPLE);
        assert_eq!(result, 25272);
    }
}
