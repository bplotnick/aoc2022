use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    part1();
    part2();
}

fn sum_elves() -> Vec<Vec<u32>> {
    let mut elves = Vec::<Vec<u32>>::new();

    let file = File::open("input.txt").expect("can't find input.txt");
    let reader = BufReader::new(file);

    let mut current_elf = Vec::<u32>::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        if line.is_empty() {
            elves.push(current_elf);
            current_elf = Vec::<u32>::new();
        } else {
            current_elf.push(line.parse::<u32>().unwrap());
        }
    }
    // no newline on last elf
    elves.push(current_elf);
    return elves;
}
fn part1() {
    let elves = sum_elves();

    println!(
        "{}",
        elves.iter().map(|x| x.iter().sum::<u32>()).max().unwrap()
    );
}

fn part2() {
    let elves = sum_elves();

    let mut elf_sums: Vec<u32> = elves.iter().map(|x| x.iter().sum::<u32>()).collect();
    elf_sums.sort();
    println!("{}", elf_sums[elf_sums.len() - 3..].iter().sum::<u32>());
}
