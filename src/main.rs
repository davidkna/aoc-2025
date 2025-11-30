use clap::Parser;

#[derive(clap::Parser)]
struct Args {
    day: u8,
}

fn main() {
    let args = Args::parse();
    let day = args.day;

    match day {
        1 => {
            aoc_2025::day01::run();
        }
        _ => unimplemented!("Day {day} not implemented yet"),
    }
}
