use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Op {
    num: u32,
    from: usize,
    to: usize,
}

type Stack = Vec<char>;

fn main() {
    let file = File::open("input.txt").expect("can't find input.txt");
    let mut lines = BufReader::new(file).lines().filter_map(|i| i.ok());

    let stax = build_stacks(&mut lines);

    let ops: Vec<_> = parse_ops(&mut lines);

    let mut stax_pt1 = stax.clone();
    execute_ops(&mut stax_pt1, &ops);
    println!("{}", stax_pt1.iter().map(|s| s.last().unwrap()).join(&""));

    let mut stax_pt2 = stax.clone();
    execute_ops_pt2(&mut stax_pt2, &ops);
    println!("{}", stax_pt2.iter().map(|s| s.last().unwrap()).join(&""));
}

// Modifies stax in place
fn execute_ops(stax: &mut Vec<Stack>, ops: &Vec<Op>) {
    for op in ops {
        for _ in 0..op.num {
            let a = stax[op.from - 1].pop().unwrap();
            stax[op.to - 1].push(a);
        }
    }
}

fn execute_ops_pt2(stax: &mut Vec<Stack>, ops: &Vec<Op>) {
    for op in ops {
        (0..op.num)
            .map(|_| stax[op.from - 1].pop().unwrap())
            .collect::<Vec<_>>()
            .iter()
            .rev()
            .for_each(|a| stax[op.to - 1].push(*a));
    }
}

fn build_stacks(lines: &mut impl Iterator<Item = String>) -> Vec<Stack> {
    let mut stax: Vec<Stack> = (0..9).map(|_| Vec::new()).collect();

    let mut temp_stax: Vec<Stack> = Vec::new();

    let mut line = lines.next().unwrap();
    loop {
        if line.is_empty() {
            break;
        }
        let chars = parse_line(&line);
        temp_stax.push(chars);
        line = lines.next().unwrap();
    }
    temp_stax.pop();
    while let Some(row) = temp_stax.pop() {
        for (i, c) in row.iter().enumerate() {
            if c != &' ' {
                stax[i].push(*c);
            }
        }
    }
    return stax;
}

fn parse_line(line: &str) -> Vec<char> {
    let mut char_iter = line.chars();
    let mut chars: Vec<char> = Vec::new();
    while let Some((_, c, _)) = char_iter.next_tuple() {
        chars.push(c);
        char_iter.next();
    }
    return chars;
}

fn parse_ops(lines: &mut impl Iterator<Item = String>) -> Vec<Op> {
    lines
        .map(|l| l.split_whitespace().map(String::from).collect::<Vec<_>>())
        .map(|s| Op {
            num: s[1].parse().unwrap(),
            from: s[3].parse().unwrap(),
            to: s[5].parse().unwrap(),
        })
        .collect()
}
