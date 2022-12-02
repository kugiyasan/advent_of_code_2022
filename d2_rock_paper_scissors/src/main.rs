use std::{fs::File, io::Read, str::FromStr};

#[derive(Debug, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Hand {
    fn get_point(&self) -> i32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn fight(&self, other: &Hand) -> Outcome {
        match (self, other) {
            (Hand::Rock, Hand::Paper) => Outcome::Loss,
            (Hand::Rock, Hand::Scissors) => Outcome::Win,
            (Hand::Paper, Hand::Rock) => Outcome::Win,
            (Hand::Paper, Hand::Scissors) => Outcome::Loss,
            (Hand::Scissors, Hand::Rock) => Outcome::Loss,
            (Hand::Scissors, Hand::Paper) => Outcome::Win,
            _ => Outcome::Draw,
        }
    }

    fn get_score(&self, other: &Hand) -> i32 {
        let score = match self.fight(other) {
            Outcome::Win => 6,
            Outcome::Loss => 0,
            Outcome::Draw => 3,
        };

        score + self.get_point()
    }
}

impl FromStr for Hand {
    type Err = ();
    fn from_str(s: &str) -> Result<Hand, ()> {
        match s {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scissors),
            _ => Err(()),
        }
    }
}

fn main() {
    let path = "input";
    let mut buf = String::new();
    File::open(path).unwrap().read_to_string(&mut buf).unwrap();

    let input: Vec<Vec<Hand>> = buf
        .trim()
        .split('\n')
        .map(|s| {
            s.split(' ')
                .map(|hand| hand.parse::<Hand>().unwrap())
                .collect()
        })
        .collect();

    let total_score: i32 = input
        .iter()
        .map(|hands| hands[1].get_score(&hands[0]))
        .sum();

    // let total_score: Vec<i32> = input
    //     .iter()
    //     .map(|hands| hands[1].get_score(&hands[0]))
    //     .collect();

    // println!("{:?}", &input[0..5]);
    // println!("{:?}", &total_score[0..5]);
    println!("{}", total_score);
    // println!("{:?}", input);
}
