use puzzle::DailyPuzzle;
use std::env;

mod day01;
mod day02;
mod day03;
mod puzzle;

fn main() {
    let days: Vec<Box<dyn DailyPuzzle>> = vec![
        day01::Day01::new(),
        day02::Day02::new(),
        day03::Day03::new(),
    ];
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let run_one = args[1]
            .parse::<usize>()
            .expect("should be a valid day number");
        days[run_one - 1].run();
        return;
    }

    for d in days {
        d.run();
    }
}
