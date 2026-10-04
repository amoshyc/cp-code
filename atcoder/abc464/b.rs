#![allow(unused)]

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        mut arr: [Chars; h],
    }

    let f = |arr: Vec<Vec<char>>| -> Vec<Vec<char>> {
        let mut arr = arr.clone();
        for _ in 0..arr.len() {
            if arr[arr.len() - 1].iter().all(|&c| c == '.') {
                arr.pop();
            } else {
                break;
            }
        }
        arr
    };

    // top
    arr.reverse();
    arr = f(arr);
    arr.reverse();

    // bottom
    arr = f(arr);

    // left
    arr = rotate_cw(&arr);
    arr = f(arr);
    arr = rotate_ccw(&arr);

    // right
    arr = rotate_ccw(&arr);
    arr = f(arr);
    arr = rotate_cw(&arr);

    for r in 0..arr.len() {
        println!("{}", join(&arr[r], ""));
    }
}

// clockwise
//  123        41
//  456   ->   52
//             63
// (2x3)      (3x2)
fn rotate_cw<T: Clone>(arr: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let (n, m) = (arr.len(), arr[0].len());
    let mut res = vec![vec![arr[0][0].clone(); n]; m];
    for r in 0..n {
        for c in 0..m {
            res[c][n - 1 - r] = arr[r][c].clone();
        }
    }
    res
}

// counterclockwise
//  123        36
//  456   ->   25
//             14
// (2x3)      (3x2)
fn rotate_ccw<T: Clone>(arr: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let (n, m) = (arr.len(), arr[0].len());
    let mut res = vec![vec![arr[0][0].clone(); n]; m];
    for r in 0..n {
        for c in 0..m {
            res[m - 1 - c][r] = arr[r][c].clone();
        }
    }
    res
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
