use std::{i32, ops::Range};

use super::puzzle::DailyPuzzle;

const INPUT: &str = include_str!("input.txt");

#[derive(Clone)]
pub struct Day02 {}

impl Day02 {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }

    fn parse(&self, input: &str) -> Vec<Vec<i32>> {
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|raw| raw.parse::<i32>().expect("expected valid number"))
                    .collect()
            })
            .collect()
    }

    fn part1(&self, input: &str) -> String {
        self.parse(input)
            .iter()
            .map(|r| monotonic_in_range(r))
            .filter(|a| *a)
            .count()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        unimplemented!()
    }
}

impl DailyPuzzle for Day02 {
    fn name(&self) -> String {
        "Day 02".into()
    }

    fn part1(&self) -> String {
        self.part1(INPUT)
    }

    fn part2(&self) -> String {
        self.part2(INPUT)
    }
}

fn monotonic_in_range(nums: &Vec<i32>) -> bool {
    let diffs: Vec<i32> = nums.windows(2).map(|pair| pair[1] - pair[0]).collect();
    diffs.iter().all(|&d| 0 < d && d <= 3) || diffs.iter().all(|&d| -3 <= d && d < 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9";

    #[test]
    fn part1() {
        let d = Day02::new();
        let res = d.part1(EXAMPLE);

        assert_eq!(res, "2");
    }
}
