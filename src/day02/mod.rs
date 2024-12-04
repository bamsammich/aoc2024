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
        let mut out: Vec<Vec<i32>> = Vec::new();
        for line in input.lines() {
            let mut report: Vec<i32> = Vec::new();
            for raw in line.split_whitespace() {
                let v = raw.parse::<i32>().expect("expected valid number");
                report.push(v);
            }
            out.push(report);
        }

        out
    }

    fn part1(&self, input: &str) -> String {
        let mut count: i32 = 0;
        for r in self.parse(input) {
            if monotonic_in_range(&r, 1, 3) {
                count += 1;
            }
        }
        count.to_string()
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

fn monotonic_in_range(nums: &Vec<i32>, min: i32, max: i32) -> bool {
    if nums.len() <= 1 {
        return false;
    }

    if nums[1] == nums[0] {
        return false;
    }

    let increasing = nums[1] > nums[0];
    for i in 0..nums.len() - 1 {
        let diff = (nums[i + 1] - nums[i]).abs();
        if (nums[i + 1] > nums[i]) != increasing || diff < min || diff > max {
            return false;
        }
    }

    true
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
