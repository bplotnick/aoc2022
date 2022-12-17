use std::{fs::File, io::{BufReader, BufRead}, collections::HashSet};


enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Move {
    direction: Direction,
    amount: i32,
}

impl Direction {
    fn from(line: &str) -> Direction {
        match line {
            "L" => Direction::Left,
            "R" => Direction::Right,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => panic!("Invalid input"),
        }
    }
}

fn main() {
    let file = File::open("input.txt").expect("can't find input.txt");
    let lines = BufReader::new(file).lines().filter_map(|i| i.ok());
    
    part1_inner(lines);
    let file = File::open("input.txt").expect("can't find input.txt");
    let lines = BufReader::new(file).lines().filter_map(|i| i.ok());
    part2_inner(lines)
}

pub fn part1_inner<I>(lines: I) -> ()
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut tail_loc: (i32, i32) = (0,0);
    let mut head_loc: (i32, i32) = (0,0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let moves = lines.into_iter().map(
                            |l| l.as_ref()
                            .split_once(' ')
                            .map(|i| (String::from(i.0), String::from(i.1)))
                            .unwrap())
         .map(|(dir, amt)| Move{direction: Direction::from(&dir), amount: amt.parse().unwrap()})
         .collect::<Vec<Move>>();
    
    for m in moves {
        for _ in 0..m.amount {
            match m.direction {
                Direction::Left => {
                    head_loc.0 -= 1;
                    if head_loc.0.abs_diff(tail_loc.0) == 2 {
                        tail_loc = head_loc;
                        tail_loc.0 += 1;
                    }
                },
                Direction::Right => {
                    head_loc.0 += 1;
                    if head_loc.0.abs_diff(tail_loc.0) == 2 {
                        tail_loc = head_loc;
                        tail_loc.0 -= 1;
                    }
                },
                Direction::Up => {
                    head_loc.1 -= 1;
                    if head_loc.1.abs_diff(tail_loc.1) == 2 {
                        tail_loc = head_loc;
                        tail_loc.1 += 1;
                    }
                },
                Direction::Down => {
                    head_loc.1 += 1;
                    if head_loc.1.abs_diff(tail_loc.1) == 2 {
                        tail_loc = head_loc;
                        tail_loc.1 -= 1;
                    }
                },
            }
            visited.insert(tail_loc);
        }
    }
    println!("{}", visited.len());
    return ();
}

pub fn part2_inner<I>(lines: I) -> ()
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut locs: Vec<(i32, i32)> = vec![(0,0);10];
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let moves = lines.into_iter().map(
                            |l| l.as_ref()
                            .split_once(' ')
                            .map(|i| (String::from(i.0), String::from(i.1)))
                            .unwrap())
         .map(|(dir, amt)| Move{direction: Direction::from(&dir), amount: amt.parse().unwrap()})
         .collect::<Vec<Move>>();
    
    for m in moves {
        for _ in 0..m.amount {
            match m.direction {
                Direction::Left => locs[0].0 -= 1,
                Direction::Right => locs[0].0 += 1,
                Direction::Up => locs[0].1 -= 1,
                Direction::Down => locs[0].1 += 1,
            }
            for i in 1..locs.len() {
                let diff = (locs[i-1].0 - locs[i].0, locs[i-1].1 - locs[i].1);
                if diff.0.abs() >=2 || diff.1.abs() >= 2 {
                    locs[i] = locs[i-1];
                } else {
                    continue;
                }

                if diff.0 == 2 {
                    locs[i].0 -= 1;
                } else if diff.0 == -2 {
                    locs[i].0 += 1;
                }

                if diff.1 == 2 {
                    locs[i].1 -= 1;
                } else if diff.1 == -2 {
                    locs[i].1 += 1;
                }
            }
            visited.insert(locs[9]);
        }
    }
    println!("{}", visited.len());
    return ();
}

#[test]
fn test_case() {
    let x = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2".to_string();
    part2_inner(x.lines());
}