#![allow(unused)]

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        supply: [usize; m],
        ab: [(Usize1, usize); n],
    }

    let mut need = vec![0; m];
    for i in 0..n {
        need[ab[i].0] += ab[i].1;
    }

    let ans = (0..m).map(|i| supply[i].min(need[i]) as i64).sum::<i64>();
    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
