use regex::Regex;
use std::{
    collections::VecDeque,
    io::{self},
};

#[derive(Clone)]
struct SupplyStacks {
    stacks: Vec<VecDeque<char>>,
}

impl SupplyStacks {
    fn new(num_stacks: usize) -> Self {
        println!("SupplyStacks with {num_stacks} stacks");
        SupplyStacks {
            stacks: vec![VecDeque::new(); num_stacks],
        }
    }

    fn add_to_crate(&mut self, name: char, stack: usize) {
        self.stacks[stack].push_front(name);
    }

    fn move_crates(&mut self, from: usize, to: usize, crates: usize) {
        for _ in 0..crates {
            let c = self.stacks[from].pop_back().unwrap();
            self.stacks[to].push_back(c);
        }
    }

    fn move_stacked_crates(&mut self, from: usize, to: usize, crates: usize) {
        let mut tmp = VecDeque::new();
        for _ in 0..crates {
            let c = self.stacks[from].pop_back().unwrap();
            tmp.push_front(c);
        }

        for c in tmp.into_iter() {
            self.stacks[to].push_back(c);
        }
    }

    fn print_top(&self) {
        let mut msg = String::new();

        for stack in &self.stacks {
            if stack.is_empty() {
                continue;
            }

            msg.push(*stack.back().unwrap());
        }

        println!("{msg}");
    }
}

fn find_number_of_stacks(line: &str) -> usize {
    (line.len() + 1) / 4
}

fn find_crate_position(index: usize) -> usize {
    (index - 1) / 4
}

fn parse_line_to_stacks(line: &str, supply_stacks: &mut SupplyStacks) {
    line.chars().enumerate().for_each(|(i, c)| {
        if c.is_alphabetic() {
            let pos = find_crate_position(i);
            println!("Adding {c} to {pos}");
            supply_stacks.add_to_crate(c, pos);
        }
    });
}

fn read_line() -> Option<String> {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(0) => None,
        Ok(_) => Some(buffer),
        Err(_) => None,
    }
}

fn main() {
    let mut buffer = read_line().unwrap();
    let num_stacks = find_number_of_stacks(buffer.as_str());
    let mut stacks = SupplyStacks::new(num_stacks);

    while !buffer.starts_with(" 1") {
        parse_line_to_stacks(buffer.as_str(), &mut stacks);
        buffer = read_line().unwrap();
    }

    let _ = read_line();

    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    let mut cloned = stacks.clone();
    while let Some(mut command) = read_line() {
        command.pop();
        println!("{command}");
        let blah = re.captures(command.as_str()).unwrap();
        let num_stacks = &blah[1].parse::<usize>().unwrap();
        let from = &blah[2].parse::<usize>().unwrap();
        let to = &blah[3].parse::<usize>().unwrap();
        stacks.move_crates(*from - 1, *to - 1, *num_stacks);
        cloned.move_stacked_crates(*from - 1, *to - 1, *num_stacks);
        println!("{num_stacks} {from} {to}");
    }

    stacks.print_top();
    cloned.print_top();
}
