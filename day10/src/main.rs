use std::{fs::File, io::{BufReader, BufRead}};



fn main() {
    let file = File::open("input.txt").expect("can't find input.txt");
    let lines = BufReader::new(file).lines().filter_map(|i| i.ok());
    let ops = make_ops(lines);
    part1_inner(&ops);
    part2_inner(&ops);
}

fn make_ops<I>(lines: I) -> Vec<(u32, i32)>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    // Tuple represents value at the _end_ of the clk
    let mut cur_clk: u32 = 0;
    let mut cur_val: i32 = 1;
    let mut ops: Vec<(u32, i32)> = vec![(cur_clk, cur_val)];
    for l in lines.into_iter() {
        if l.as_ref().starts_with("addx") {
            cur_val += l.as_ref().split_once(' ').unwrap().1.parse::<i32>().unwrap();
            cur_clk += 2;
            ops.push((cur_clk, cur_val))
        } else {
            assert!(l.as_ref().starts_with("noop"));
            cur_clk += 1;
        }
    }
    return ops;
}

fn part1_inner(ops: &Vec<(u32, i32)>) -> () {
    let cycles = vec![20, 60, 100, 140, 180, 220];
    let mut total = 0;
    for c in cycles {
        let foo = ops.binary_search_by_key(&c, |&(a,_)| a);
        let idx = match foo {
            Ok(s) => s-1,
            Err(s) => s-1,
        };
        total += c as i32*ops[idx].1;
    }
    println!("{}", total);
}

fn part2_inner(ops: &Vec<(u32, i32)>) -> () {
    for c in 1..=240 {
        let foo = ops.binary_search_by_key(&c, |&(a,_)| a);
        let idx = match foo {
            Ok(s) => s-1,
            Err(s) => s-1,
        };
        if (c as i32)%40-1 >= ops[idx].1-1 && (c as i32)%40-1 <= ops[idx].1 + 1{
            print!("#");
        } else {
            print!(".");
        }

        if c % 40 == 0 {
            println!("");
        }
    }
}

#[test]
fn test_fn() {
    let x = "noop
addx 3
addx -5".to_string();
    let ops = make_ops(x.lines());
    part1_inner(&ops);
}