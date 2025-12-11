use bstr::ByteSlice;
use fnv::FnvHashMap;

pub const INPUT: &[u8] = include_bytes!("../../inputs/day11.txt");

fn count_paths<'a>(
    node: &'a [u8],
    graph: &FnvHashMap<&'a [u8], Vec<&'a [u8]>>,
    cache: &mut FnvHashMap<&'a [u8], u32>,
) -> u32 {
    if let Some(&cached) = cache.get(node) {
        return cached;
    }

    let mut total = 0;
    if let Some(neighbors) = graph.get(node) {
        for &neighbor in neighbors {
            total += count_paths(neighbor, graph, cache);
        }
    }

    cache.insert(node, total);
    total
}

pub fn part_1(input: &[u8]) -> u32 {
    let graph = input
        .lines()
        .map(|line| {
            let (input, targets) = line.split_once_str(": ").unwrap();
            let targets: Vec<&[u8]> = targets.split_str(" ").collect();
            (input, targets)
        })
        .collect::<FnvHashMap<_, _>>();

    let mut cache: FnvHashMap<&[u8], u32> = FnvHashMap::default();
    cache.insert(b"out", 1);

    count_paths(b"you", &graph, &mut cache)
}

#[derive(Debug, Clone, Copy)]
struct CacheEntryPart2 {
    contains_only_fft: u64,
    contains_only_dac: u64,
    contains_both: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct StatePart2<'a> {
    name: &'a [u8],
    fft_included: bool,
    dac_included: bool,
}

fn count_paths_part2<'a>(
    state: StatePart2<'a>,
    graph: &FnvHashMap<&'a [u8], Vec<&'a [u8]>>,
    cache: &mut FnvHashMap<StatePart2<'a>, CacheEntryPart2>,
) -> CacheEntryPart2 {
    if let Some(&cached) = cache.get(&state) {
        return cached;
    }

    if state.name == b"out" {
        let entry = CacheEntryPart2 {
            contains_only_fft: if state.fft_included && !state.dac_included {
                1
            } else {
                0
            },
            contains_only_dac: if state.dac_included && !state.fft_included {
                1
            } else {
                0
            },
            contains_both: if state.fft_included && state.dac_included {
                1
            } else {
                0
            },
        };
        cache.insert(state, entry);
        return entry;
    }

    let mut entry = CacheEntryPart2 {
        contains_only_fft: 0,
        contains_only_dac: 0,
        contains_both: 0,
    };

    if let Some(neighbors) = graph.get(state.name) {
        for &neighbor in neighbors {
            let neighbor_state = StatePart2 {
                name: neighbor,
                fft_included: state.fft_included || neighbor == b"fft",
                dac_included: state.dac_included || neighbor == b"dac",
            };
            let neighbor_entry = count_paths_part2(neighbor_state, graph, cache);
            entry.contains_only_fft += neighbor_entry.contains_only_fft;
            entry.contains_only_dac += neighbor_entry.contains_only_dac;
            entry.contains_both += neighbor_entry.contains_both;
        }
    }
    cache.insert(state, entry);
    entry
}

pub fn part_2(input: &[u8]) -> u64 {
    let graph = input
        .lines()
        .map(|line| {
            let (input, targets) = line.split_once_str(": ").unwrap();
            let targets: Vec<&[u8]> = targets.split_str(" ").collect();
            (input, targets)
        })
        .collect::<FnvHashMap<_, _>>();

    let mut cache: FnvHashMap<StatePart2, CacheEntryPart2> = FnvHashMap::default();

    let start = StatePart2 {
        name: b"svr",
        fft_included: false,
        dac_included: false,
    };
    let result = count_paths_part2(start, &graph, &mut cache);
    result.contains_both
}

pub fn run() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &[u8] = b"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    const EXAMPLE_2: &[u8] = b"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_part_1() {
        let result = part_1(EXAMPLE_1);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(EXAMPLE_2);
        assert_eq!(result, 2);
    }
}
