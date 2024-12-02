pub trait DailyPuzzle {
    fn name(&self) -> String;
    fn test01(&self) -> String;
    fn test02(&self) -> String;
    fn run(&self) {
        println!("=== {} ===", self.name());
        println!("- Test 01: {}", self.test01());
        println!("- Test 02: {}", self.test02());
    }
}
