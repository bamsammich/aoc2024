pub trait DailyPuzzle {
    fn name(&self) -> String;
    fn test01(&self) -> String;
    fn test02(&self) -> String;
}
