use std::{fs::File, io::Read, str::FromStr};

#[derive(Clone, Debug, PartialEq)]
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
        self.fight(other).get_point() + self.get_point()
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

impl Outcome {
    fn get_point(&self) -> i32 {
        match self {
            Outcome::Win => 6,
            Outcome::Loss => 0,
            Outcome::Draw => 3,
        }
    }

    fn find_other_hand(&self, other: &Hand) -> Hand {
        match (self, other) {
            (Outcome::Win, Hand::Rock) => Hand::Paper,
            (Outcome::Win, Hand::Paper) => Hand::Scissors,
            (Outcome::Win, Hand::Scissors) => Hand::Rock,
            (Outcome::Loss, Hand::Rock) => Hand::Scissors,
            (Outcome::Loss, Hand::Paper) => Hand::Rock,
            (Outcome::Loss, Hand::Scissors) => Hand::Paper,
            (Outcome::Draw, _) => other.clone(),
        }
    }

    fn get_score(&self, other: &Hand) -> i32 {
        self.find_other_hand(other).get_point() + self.get_point()
    }
}

impl FromStr for Outcome {
    type Err = ();
    fn from_str(s: &str) -> Result<Outcome, ()> {
        match s {
            "X" => Ok(Outcome::Loss),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(()),
        }
    }
}

fn part1(buf: &str) {
    let input: Vec<(Hand, Hand)> = buf
        .trim()
        .split('\n')
        .map(|s| {
            let line: Vec<_> = s.split(' ').collect();
            let hand = line[0].parse::<Hand>().unwrap();
            let other = line[1].parse::<Hand>().unwrap();
            (hand, other)
        })
        .collect();

    let total_score: i32 = input
        .iter()
        .map(|(other, hand)| hand.get_score(&other))
        .sum();

    println!("{}", total_score);
}

fn part2(buf: &str) {
    let input: Vec<(Hand, Outcome)> = buf
        .trim()
        .split('\n')
        .map(|s| {
            let line: Vec<_> = s.split(' ').collect();
            let hand = line[0].parse::<Hand>().unwrap();
            let outcome = line[1].parse::<Outcome>().unwrap();
            (hand, outcome)
        })
        .collect();

    let total_score: i32 = input
        .iter()
        .map(|(hand, outcome)| outcome.get_score(&hand))
        .sum();

    println!("{}", total_score);
}

fn main() {
    let path = "input";
    let mut buf = String::new();
    File::open(path).unwrap().read_to_string(&mut buf).unwrap();

    // part1(&buf);
    part2(&buf);
}
