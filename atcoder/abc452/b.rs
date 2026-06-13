#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    }

    let mut res = vec![vec!['#'; w]; h];
    for r in 1..(h - 1) {
        for c in 1..(w - 1) {
            res[r][c] = '.';
        }
    }

    for r in 0..h {
        println!("{}", join(&res[r], ""));
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
