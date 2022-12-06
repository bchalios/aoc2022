use std::io::{self, BufRead};

struct Section {
    start: u32,
    end: u32,
}

impl Section {
    fn new_from_range(range: &str) -> Self {
        let range: Vec<u32> = range
            .split('-')
            .map(|repr| repr.parse::<u32>().expect("Wrong input"))
            .collect();

        Section {
            start: range[0],
            end: range[1],
        }
    }

    fn contains(&self, other: &Section) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Section) -> bool {
        (self.start <= other.start && other.start <= self.end)
            || (other.start <= self.start && self.start <= other.end)
    }
}

fn main() {
    let overlapping = io::stdin().lock().lines().fold(0, |acc, line| {
        let line = line.unwrap();

        let mut split = line.split(',');
        let first = Section::new_from_range(split.next().unwrap());
        let second = Section::new_from_range(split.next().unwrap());

        //if first.contains(&second) || second.contains(&first) {
        if first.overlaps(&second) {
            acc + 1
        } else {
            acc
        }
    });

    println!("{}", overlapping);
}
