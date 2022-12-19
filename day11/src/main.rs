use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, Mul},
};

use itertools::Itertools;

struct Monkey {
    items: VecDeque<u64>,
    op: String,
    test: u16,
    true_monkey: String,
    false_monkey: String,
}

impl Monkey {
    fn eval_next(&mut self) -> Option<(u64, String)> {
        let item = self.items.pop_front()?;

        let mut op = self.op.split_whitespace();
        let first_operand = match op.next() {
            Some("old") => item,
            Some(x) => x.parse().unwrap(),
            None => return None,
        };
        let operator = match op.next() {
            Some("*") => Mul::mul,
            Some("+") => Add::add,
            None | Some(&_) => return None,
        };
        let second_operand = match op.next() {
            Some("old") => item,
            Some(x) => x.parse().unwrap(),
            None => return None,
        };

        let item = operator(first_operand, second_operand) / 3;
        if item % self.test as u64 == 0 {
            Some((item, self.true_monkey.clone()))
        } else {
            Some((item, self.false_monkey.clone()))
        }
    }
}

fn main() {
    let file = File::open("input.txt").expect("can't find input.txt");
    let lines = BufReader::new(file).lines().filter_map(|i| i.ok());

    let mut monkeys = Vec::<Monkey>::new();

    for mut chunk in lines.chunks(7).into_iter() {
        chunk.next(); // monkeynum

        let itemstr = chunk.next().unwrap();
        let items = itemstr
            .as_str()
            .split_once(":")
            .unwrap()
            .1
            .trim()
            .split(",")
            .map(|x| x.trim().to_owned().parse().unwrap())
            .collect::<VecDeque<u64>>();
        let opstr = chunk.next().unwrap();
        let op = opstr.as_str().split_once("= ").unwrap().1.to_owned();

        let teststr = chunk.next().unwrap();
        let test: u16 = teststr
            .as_str()
            .rsplit_once(" ")
            .unwrap()
            .1
            .parse()
            .unwrap();

        let true_monkey = chunk.next().unwrap().rsplit_once(" ").unwrap().1.to_owned();
        let false_monkey = chunk.next().unwrap().rsplit_once(" ").unwrap().1.to_owned();

        let monkey = Monkey {
            items,
            op,
            test,
            true_monkey,
            false_monkey,
        };
        monkeys.push(monkey);
        chunk.next();
    }

    let mut inspections = vec![0u32; monkeys.len()];
    for _ in 0..20 {
        for m in 0..monkeys.len() {
            let cur_monkey = &mut monkeys[m];
            let mut results = Vec::<(u64, String)>::new();
            while let Some(result) = cur_monkey.eval_next() {
                inspections[m] += 1;
                results.push(result);
            }
            for (item, dest) in results {
                monkeys[dest.parse::<usize>().unwrap()]
                    .items
                    .push_back(item);
            }
        }
    }
    inspections.sort();
    inspections.reverse();
    println!("{}", inspections[0] * inspections[1]);
}
