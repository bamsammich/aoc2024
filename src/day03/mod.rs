use super::puzzle::DailyPuzzle;
use regex::Regex;

const INPUT: &str = include_str!("input.txt");

#[derive(Clone)]
pub struct Day03 {}

impl Day03 {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }

    fn part1(&self, input: &str) -> String {
        add_multiples(input).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let re_enabled = Regex::new(r"do\(\)(?P<enabled>.*?)don't\(\)").unwrap();
        re_enabled
            .captures_iter(format!("do(){input}don't()").as_str())
            .map(|cap| add_multiples(cap.name("enabled").unwrap().as_str()))
            .sum::<i32>()
            .to_string()
    }
}

impl DailyPuzzle for Day03 {
    fn name(&self) -> String {
        "Day 03".into()
    }

    fn part1(&self) -> String {
        self.part1(INPUT)
    }

    fn part2(&self) -> String {
        self.part2(INPUT)
    }
}

fn add_multiples(input: &str) -> i32 {
    let re = Regex::new(r"mul\((?P<left>\d+),(?P<right>\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            cap.name("left")
                .unwrap()
                .as_str()
                .parse::<i32>()
                .expect("valid number")
                * cap
                    .name("right")
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .expect("valid number")
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part1() {
        let d = Day03::new();

        assert_eq!(d.part1(EXAMPLE), "161");
    }

    #[test]
    fn part2() {
        let d = Day03::new();

        assert_eq!(d.part2(EXAMPLE), "48");
    }
}
