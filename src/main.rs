use clap::Parser;

#[derive(clap::Parser)]
struct Args {
    day: u8,
}

fn main() {
    let args = Args::parse();
    let day = args.day;

    match day {
        1 => aoc_2025::day01::run(),
        2 => aoc_2025::day02::run(),
        3 => aoc_2025::day03::run(),
        4 => aoc_2025::day04::run(),
        5 => aoc_2025::day05::run(),
        6 => aoc_2025::day06::run(),
        7 => aoc_2025::day07::run(),
        8 => aoc_2025::day08::run(),
        9 => aoc_2025::day09::run(),
        _ => unimplemented!("Day {day} not implemented yet"),
    }
}
