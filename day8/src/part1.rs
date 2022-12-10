use std::cmp;

use crate::board::Board;

pub fn count_viz(board: &Board) -> u32 {
    let matrix = &board.matrix;
    let size = matrix.len(); // We know it's square
    let mut results = vec![vec![vec![false; 4]; size]; size];
    for i in 0..size {
        results[0][i].fill(true);
        results[size - 1][i].fill(true);
        results[i][0].fill(true);
        results[i][size - 1].fill(true);
    }

    for first in 1..size - 1 {
        let mut max_vec = vec![
            matrix[first][0],
            matrix[first][size - 0 - 1],
            matrix[0][first],
            matrix[size - 0 - 1][first],
        ];
        for second in 1..size - 1 {
            let curval = matrix[first][second];
            if curval > max_vec[0] {
                results[first][second][0] = true;
            }
            max_vec[0] = cmp::max(curval, max_vec[0]);

            let curval = matrix[first][size - second - 1];
            if curval > max_vec[1] {
                results[first][size - second - 1][1] = true;
            }
            max_vec[1] = cmp::max(curval, max_vec[1]);

            let curval = matrix[second][first];
            if curval > max_vec[2] {
                results[second][first][2] = true;
            }
            max_vec[2] = cmp::max(curval, max_vec[2]);

            let curval = matrix[size - second - 1][first];
            if curval > max_vec[3] {
                results[size - second - 1][first][3] = true;
            }
            max_vec[3] = cmp::max(curval, max_vec[3]);
        }
        // println!("{:?}", results)
    }

    let viz = results
        .iter()
        .map(|i| {
            i.iter()
                .map(|j| j.iter().any(|k| *k))
                .collect::<Vec<bool>>()
        })
        .collect::<Vec<Vec<bool>>>();
    // println!("{:?}", viz);
    // for i in 0..size {
    //     for j in 0..size {
    //         print!("({}, {})", results[i][j].iter().any(|x| *x), matrix[i][j]);
    //     }
    //     println!();
    // }
    // println!("{:?}", matrix.iter().next().unwrap());
    // println!("{:?}", results.iter().next().unwrap());
    let x = viz
        .iter()
        .map(|i| i.iter().map(|j| *j as u32).sum::<u32>())
        .sum::<u32>();
    x
}

#[test]
fn test_case() {
    let x = vec!["30373", "25512", "65332", "33549", "35390"];
    let board = Board::from(&mut x.into_iter());
    println!("{}", count_viz(&board));
}
