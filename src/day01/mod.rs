use super::puzzle::DailyPuzzle;

const INPUT: &str = include_str!("input.txt");

#[derive(Clone)]
pub struct Day01 {}

impl Day01 {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }

    fn parse(&self, input: &str) -> (Vec<i32>, Vec<i32>) {
        let mut l: Vec<i32> = Vec::new();
        let mut r: Vec<i32> = Vec::new();

        for line in input.lines() {
            for (i, p) in line.split_whitespace().enumerate() {
                match i {
                    0 => l.push(p.parse::<i32>().unwrap()),
                    1 => r.push(p.parse::<i32>().unwrap()),
                    _ => panic!("found too many values"),
                }
            }
        }

        (l, r)
    }

    fn part1(&self, input: &str) -> String {
        let (mut l, mut r) = self.parse(input);
        l.sort();
        r.sort();

        let mut val: i32 = 0;
        for i in 0..l.len() {
            val += (l[i] - r[i]).abs();
        }

        val.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (l, r) = self.parse(input);

        let mut sum: i32 = 0;

        for x in &l {
            let mut times: i32 = 0;
            for y in &r {
                if x == y {
                    times += 1;
                }
            }
            sum += x * times
        }

        sum.to_string()
    }
}

impl DailyPuzzle for Day01 {
    fn name(&self) -> String {
        "Day 01".into()
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

    const EXAMPLE: &str = "3   4
    4   3
    2   5
    1   3
    3   9
    3   3";

    #[test]
    fn part1() {
        let d1 = Day01::new();
        let res = d1.part1(EXAMPLE);

        println!("stuff");
        assert_eq!(res, "11");
    }

    #[test]
    fn part2() {
        let d1 = Day01::new();
        let res = d1.part2(EXAMPLE);

        println!("other stuff");
        assert_eq!(res, "31");
    }
}
