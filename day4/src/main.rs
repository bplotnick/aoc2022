use std::{fs::File, io::{BufReader, BufRead}};


struct Section {
    start: u32,
    end: u32,
}

impl Section {
    fn from(s: &str) -> Self {
        let (first, second) = s.split_once('-').unwrap();
        let section = Section { start: first.parse().unwrap(), end: second.parse().unwrap()};
        section
    }
}

fn main() {
    let file = File::open("input.txt").expect("can't find input.txt");
    let reader = BufReader::new(file);

    let lines: Vec<(Section, Section)> = reader.lines()
        .filter_map(|i| i.ok())
        .map(|l| parse_line(&l))
        .collect();
    
    println!("{}", lines.iter()
        .map(|l| overlaps(&l.0, &l.1) as u32)
        .sum::<u32>());

    println!("{}", lines.iter()
        .map(|l| overlaps2(&l.0, &l.1) as u32)
        .sum::<u32>());
}

fn parse_line(line: &str) -> (Section, Section) {
    let (first, second) = line.split_once(',').unwrap();
    return (Section::from(first), Section::from(second));
}

fn overlaps(sec1: &Section, sec2: &Section) -> bool {
    return (sec1.start >= sec2.start && sec1.end <= sec2.end) || (sec2.start >= sec1.start && sec2.end <= sec1.end);
}

fn overlaps2(sec1: &Section, sec2: &Section) -> bool {
    return !((sec1.start > sec2.end) || (sec2.start > sec1.end))
}