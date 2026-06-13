#![allow(unused)]

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); n],
    }

    let mut cnt_a = vec![0 as isize; m];
    let mut cnt_b = vec![0 as isize; m];
    for i in 0..n {
        let (a, b) = ab[i];
        cnt_a[a] += 1;
        cnt_b[b] += 1;
    }

    let ans = (0..m).map(&|i| cnt_b[i] - cnt_a[i]).collect::<Vec<isize>>();
    println!("{}", join(&ans, "\n"));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
