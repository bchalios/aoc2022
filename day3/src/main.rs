use std::collections::HashSet;
use std::io::{self, BufRead};

struct Compartment(HashSet<char>);

impl Compartment {
    fn new(raw: &str) -> Self {
        let mut inner = HashSet::new();

        raw.chars().for_each(|c| {
            let _ = inner.insert(c);
        });

        Self(inner)
    }

    fn find_common(&self, other: &str) -> Option<char> {
        for c in other.chars() {
            if self.0.contains(&c) {
                return Some(c);
            }
        }

        None
    }

    fn intersection(&self, other: &Compartment) -> Self {
        let mut inter = HashSet::new();

        for first in &self.0 {
            if other.0.contains(first) {
                inter.insert(*first);
            }
        }

        Self(inter)
    }
}

fn item_priority(item: char) -> u32 {
    if item.is_uppercase() {
        item as u32 - 'A' as u32 + 27
    } else {
        item as u32 - 'a' as u32 + 1
    }
}

fn main() {
    /*
    let sum = io::stdin().lock().lines().fold(0, |acc, line| {
        let line = line.unwrap();
        let (first, second) = line.split_at(line.len() / 2);
        let first = Compartment::new(first);

        let common = first.find_common(second).unwrap();
        acc + item_priority(common)
    });

    println!("{}", sum);
    */

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut sum = 0;
    while let Some(first) = lines.next() {
        let first = Compartment::new(&first.unwrap());
        let second = Compartment::new(&lines.next().unwrap().unwrap());
        let third = Compartment::new(&lines.next().unwrap().unwrap());

        let intersection: Vec<char> = first
            .intersection(&second)
            .intersection(&third)
            .0
            .into_iter()
            .collect();

        sum += item_priority(intersection[0]);
    }

    println!("{}", sum);
}
