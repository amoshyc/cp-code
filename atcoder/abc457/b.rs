#![allow(unused)]

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
    }

    let mut arr = vec![vec![]; n];
    for i in 0..n {
        input! {
            l: usize,
            dat: [usize; l],
        }
        arr[i] = dat;
    }

    input! {
        x: Usize1,
        y: Usize1,
    }

    println!("{}", arr[x][y]);
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
