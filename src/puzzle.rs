pub trait DailyPuzzle {
    fn name(&self) -> String;
    fn part1(&self) -> String;
    fn part2(&self) -> String;
    fn run(&self) {
        println!("=== {} ===", self.name());
        println!("- Part 1: {}", self.part1());
        println!("- Part 2: {}", self.part2());
    }
}
