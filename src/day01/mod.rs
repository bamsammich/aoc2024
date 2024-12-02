use super::puzzle::DailyPuzzle;

#[derive(Clone)]
pub struct Day01 {
    input: String,
}

impl Day01 {
    pub fn new() -> Self {
        let input = include_str!("input.txt").to_string();
        Self { input }
    }
}

impl DailyPuzzle for Day01 {
    fn test01(&self) -> String {
        let mut l: Vec<i32> = Vec::new();
        let mut r: Vec<i32> = Vec::new();

        for line in self.input.lines() {
            for (i, p) in line.split_whitespace().enumerate() {
                match i {
                    0 => l.push(p.parse::<i32>().unwrap()),
                    1 => r.push(p.parse::<i32>().unwrap()),
                    _ => panic!("found too many values"),
                }
            }
        }

        l.sort();
        r.sort();

        let mut val: i32 = 0;
        for i in 0..l.len() {
            val += (l[i] - r[i]).abs();
        }

        val.to_string()
    }

    fn test02(&self) -> String {
        String::new()
    }
}
