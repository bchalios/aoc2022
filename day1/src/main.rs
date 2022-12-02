use std::io::{self, BufRead};

fn main() {
    let mut calories = vec![];
    let mut next = 0;

    io::stdin().lock().lines().for_each(|line| {
        let line = line.unwrap();
        if line.is_empty() {
            calories.push(next);
            next = 0;
        } else {
            next += line.parse::<u32>().unwrap();
        }
    });
    calories.push(next);

    println!("{:?}", calories.iter().max());
    calories.sort_by(|a, b| b.partial_cmp(a).unwrap());
    println!("{}", calories[0] + calories[1] + calories[2]);
}
