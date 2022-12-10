mod part1;
mod part2;
mod board;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{part1::count_viz, board::Board, part2::find_best_viz};

fn main() {
    let file = File::open("input.txt").expect("can't find input.txt");
    let lines = BufReader::new(file)
        .lines()
        .filter_map(|i| i.ok())
        .collect::<Vec<String>>();
    let board = Board::from(&mut lines.iter());
    println!("{}", count_viz(&board));
    find_best_viz(&board);
}