use std::io::{self, BufRead};

enum Selection {
    Rock,
    Paper,
    Scissors,
}

impl Selection {
    fn value(&self) -> u32 {
        match *self {
            Selection::Rock => 1,
            Selection::Paper => 2,
            Selection::Scissors => 3,
        }
    }

    fn play(&self, other: &Selection) -> u32 {
        match self {
            Selection::Rock => match other {
                Selection::Rock => 3 + self.value(),
                Selection::Paper => self.value(),
                Selection::Scissors => 6 + self.value(),
            },
            Selection::Paper => match other {
                Selection::Rock => 6 + self.value(),
                Selection::Paper => 3 + self.value(),
                Selection::Scissors => self.value(),
            },
            Selection::Scissors => match other {
                Selection::Rock => self.value(),
                Selection::Paper => 6 + self.value(),
                Selection::Scissors => 3 + self.value(),
            },
        }
    }

    fn play_to_win(&self) -> u32 {
        6 + match self {
            Selection::Rock => Selection::Paper.value(),
            Selection::Paper => Selection::Scissors.value(),
            Selection::Scissors => Selection::Rock.value(),
        }
    }

    fn play_to_draw(&self) -> u32 {
        self.value() + 3
    }

    fn play_to_loose(&self) -> u32 {
        match self {
            Selection::Rock => Selection::Scissors.value(),
            Selection::Paper => Selection::Rock.value(),
            Selection::Scissors => Selection::Paper.value(),
        }
    }
}

impl TryFrom<&str> for Selection {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" | "X" => Ok(Selection::Rock),
            "B" | "Y" => Ok(Selection::Paper),
            "C" | "Z" => Ok(Selection::Scissors),
            _ => Err("WTF is that?".to_string()),
        }
    }
}

fn main() {
    let rounds: Vec<(Selection, Selection)> = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let tokens = line.split_once(' ').unwrap();
            (tokens.0.try_into().unwrap(), tokens.1.try_into().unwrap())
        })
        .collect();

    let points = rounds
        .iter()
        .fold(0, |acc, (other, me)| acc + me.play(other));

    println!("{}", points);

    let points = io::stdin().lock().lines().fold(0, |acc, line| {
        let line = line.unwrap();
        println!("Folding line: {}", line);
        let tokens = line.split_once(' ').unwrap();
        let other: Selection = tokens.0.try_into().unwrap();

        let val = match tokens.1 {
            "X" => other.play_to_loose(),
            "Y" => other.play_to_draw(),
            "Z" => other.play_to_win(),
            _ => panic!("Fuck you Elf"),
        };
        println!("{}", val);
        acc + val
    });

    println!("{}", points);
}
