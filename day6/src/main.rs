use itertools::Itertools;
use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("input.txt").expect("can't find input.txt");
    let lines = BufReader::new(file).lines().filter_map(|i| i.ok());

    let pos: Vec<usize> = lines.map(|l| 
        l.chars().tuple_windows::<(_,_,_,_)>().find_position(|x|
            vec![x.0, x.1, x.2, x.3].iter().all_unique()
        ).unwrap().0 + 4
    ).collect();
    println!("{:?}", pos);
}

fn part2() {
    let file = File::open("input.txt").expect("can't find input.txt");
    let lines = BufReader::new(file).lines().filter_map(|i| i.ok());

    let pos: Vec<usize> = lines.map(|l| 
        l.as_bytes().windows(14).find_position(|x|
            x.iter().all_unique()
        ).unwrap().0 + 14
    ).collect();
    println!("{:?}", pos);
}