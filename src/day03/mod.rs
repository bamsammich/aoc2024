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
        let re = Regex::new(r"mul\((?P<left>\d+),(?P<right>\d+)\)").unwrap();
        let v: i32 = re
            .captures_iter(input)
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
            .sum();
        v.to_string()
    }

    fn part2(&self, input: &str) -> String {
        unimplemented!()
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

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn part1() {
        let d = Day03::new();

        assert_eq!(d.part1(EXAMPLE), "161");
    }

    // #[test]
    // fn part2() {
    //     let d = Day03::new();

    //     assert_eq!(d.part2(EXAMPLE), "4");
    // }
}
