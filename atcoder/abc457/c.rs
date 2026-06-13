#![allow(unused)]

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: i64,
    }

    let k = k - 1;

    let mut arr = vec![vec![]; n];
    for i in 0..n {
        input! {
            l: usize,
            dat: [usize; l],
        }
        arr[i] = dat;
    }

    input! {
        c: [i64; n],
    }

    let mut p = 0;
    for i in 0..n {
        let seq = arr[i].len() as i64;

        if p <= k && k < p + c[i] * seq {
            let r = (k - p) % seq;
            println!("{}", arr[i][r as usize]);
            return;
        }

        p += c[i] * seq;
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
