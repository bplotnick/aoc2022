use std::fs::File;
use std::io::{prelude::*, BufReader, self};
use std::str::FromStr;

#[derive(PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Shape {
    type Err = ();
    fn from_str(input: &str) -> Result<Shape, Self::Err> {
        match input {
            "X" | "A" => Ok(Shape::Rock),
            "Y" | "B" => Ok(Shape::Paper),
            "Z" | "C" => Ok(Shape::Scissors),
            _      => Err(()),
        }
    }
}

impl Shape {
    fn beat_by(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }

    fn beats(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }

    fn value(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

enum RoundResult {
    Win,
    Lose,
    Draw,
}

impl FromStr for RoundResult {
    type Err = ();
    fn from_str(input: &str) -> Result<RoundResult, Self::Err> {
        match input {
            "X" => Ok(RoundResult::Lose),
            "Y" => Ok(RoundResult::Draw),
            "Z" => Ok(RoundResult::Win),
            _      => Err(()),
        }
    }
}

fn main() {
    part1();
    part2();
}

fn part2() {
    let file = File::open("input.txt").expect("can't find input.txt");
    let reader = BufReader::new(file);

    println!("{}", reader.lines()
        .map(|l| {
            let line = l.unwrap();
            let (first, second) = line.split_once(" ").unwrap();
            eval_round_pt2(Shape::from_str(first).unwrap(), RoundResult::from_str(second).unwrap())
        })
        .sum::<u32>());
}

fn eval_round_pt2(theirs: Shape, round_result: RoundResult) -> u32 {
    let mut score = 0;

    match round_result {
        RoundResult::Win => {
            score += 6;
            score += theirs.beat_by().value();
        },
        RoundResult::Lose => {
            score += 0;
            score += theirs.beats().value();
        },
        RoundResult::Draw => {
            score += 3;
            score += theirs.value();
        },
    }
    score
}


fn part1() {
    let file = File::open("input.txt").expect("can't find input.txt");
    let reader = BufReader::new(file);

    println!("{}", reader.lines()
        .map(|l| {
            let line = l.unwrap();
            let (first, second) = line.split_once(" ").unwrap();
            eval_round_pt1(Shape::from_str(first).unwrap(), Shape::from_str(second).unwrap())
        })
        .sum::<u32>());
}

fn eval_round_pt1(theirs: Shape, ours: Shape) -> u32 {
    let mut score = 0;
    score += ours.value();
    if theirs == ours {
        score += 3;
    } else if theirs.beat_by() == ours {
        score += 6;
    } else if theirs.beats() == ours {
        score += 0;
    }

    return score;
}