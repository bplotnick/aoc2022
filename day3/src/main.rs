use itertools::Itertools;

use std::{fs::File, io::{prelude::*, BufReader}, collections::HashMap};

fn main() {
    part1();
    part2()
}

fn part1() {
    let file = File::open("input.txt").expect("can't find input.txt");
    let reader = BufReader::new(file);

    part1_inner(reader.lines().filter_map(|i| i.ok()));
}

fn part2() {
    let file = File::open("input.txt").expect("can't find input.txt");
    let reader = BufReader::new(file);

    part2_inner(reader.lines().filter_map(|i| i.ok()));
}

fn part1_inner<S: AsRef<str>>(reader: impl Iterator<Item = S>) {
    let dupes: Vec<_> = reader
        .map(|l| l.as_ref().chars().collect::<Vec<_>>())
        .map(|line| {
            let first_half_hist = line[..line.len()/2].iter()
                .map(|c| (c.to_owned() as char, 1))
                .collect::<HashMap<char, u8>>();
            let second_half = &line[line.len()/2..];
            for c in second_half.iter() {
                if first_half_hist.contains_key(c) {
                    return Ok(c.to_owned() as char);
                }
            }
            return Err("No duplicate found".to_owned());
        }).filter_map(|i| i.ok()).collect();
    
    let total: u32 = dupes.iter()
        .map(|c| get_priority(c.clone()) as u32)
        .sum();
    
    println!("{:?}", total);
}

fn part2_inner<S: AsRef<str>>(reader: impl Iterator<Item = S>) {
    let dupes: Vec<_> = reader
        .map(|l| l.as_ref().chars().collect::<Vec<_>>())
        .tuples::<(_, _, _)>()
        .map(|(a, b, c)| {
            let first = a.iter().map(|c| (c.to_owned() as char, 1))
                .collect::<HashMap<char, u8>>();
            let second = b.iter().map(|c| (c.to_owned() as char, 1))
                .collect::<HashMap<char, u8>>();
            for cur in c {
                if first.contains_key(&cur) && second.contains_key(&cur) {
                    return Ok(cur.to_owned() as char);
                }
            }
            return Err("No duplicate found".to_owned());
        }).filter_map(|i| i.ok()).collect();

    let total: u32 = dupes.iter()
        .map(|c| get_priority(c.clone()) as u32)
        .sum();
    println!("{:?}", total);

}

fn get_priority(c: char) -> u8 {
    let cu = c as u8;
    if (cu >= b'A') && (cu <= b'Z') {
        return cu - b'A' + 27
    } else {
        return cu - b'a' + 1;
    }
}

#[test]
fn test_case() {
    let x = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw".to_string();
    part1_inner(x.lines());
}