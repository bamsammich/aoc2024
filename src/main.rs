use puzzle::DailyPuzzle;

mod day01;
mod puzzle;

fn main() {
    let d = day01::Day01::new();
    println!("{}", d.test01());
    println!("{}", d.test02());
}
