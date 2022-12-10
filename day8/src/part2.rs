use std::cmp;

use crate::board::Board;

pub fn find_best_viz(board: &Board) {
    let matrix = &board.matrix;
    let mut best: u32 = 0;
    for y in 0..matrix.len() {
        for x in 0..matrix.len() {
            best = cmp::max(best, score(matrix, y, x));
        }
    }
    println!("{}", best);
}

fn score(matrix: &Vec<Vec<u32>>, y_node: usize, x_node: usize) -> u32 {
    // it's square
    let size = matrix.len();
    let cur_height = matrix[y_node][x_node];
    let mut viz: Vec<u32> = vec![0, 0, 0, 0];
    // top
    for y in (0..y_node).rev() {
        viz[0] += 1;
        if matrix[y][x_node] >= cur_height {
            break;
        }
    }
    // bottom
    for y in y_node+1..size {
        viz[1] += 1;
        if matrix[y][x_node] >= cur_height {
            break;
        }
    }
    // left
    for x in (0..x_node).rev() {
        viz[2] += 1;
        if matrix[y_node][x] >= cur_height {
            break;
        }
    }
    // bottom
    for x in x_node+1..size {
        viz[3] += 1;
        if matrix[y_node][x] >= cur_height {
            break;
        }
    }
    return viz.iter().product();
}

#[test]
fn test_case() {
    let x = vec!["30373", "25512", "65332", "33549", "35390"];
    let board = Board::from(&mut x.into_iter());
    println!("{}", score(&board.matrix, 3, 2));
}