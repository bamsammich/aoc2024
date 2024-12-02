use puzzle::DailyPuzzle;

mod day01;
mod puzzle;

fn main() {
    let days: Vec<Box<dyn DailyPuzzle>> = vec![day01::Day01::new()];
    for d in days {
        d.run();
    }
}
