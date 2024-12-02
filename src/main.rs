use puzzle::DailyPuzzle;

mod day01;
mod puzzle;

fn main() {
    let days: Vec<Box<dyn DailyPuzzle>> = vec![day01::Day01::new()];
    for d in days {
        println!("=== {} ===", d.name());
        println!("- Test 01: {}", d.test01());
        println!("- Test 02: {}", d.test02());
    }
}
