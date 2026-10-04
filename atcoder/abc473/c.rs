#![allow(unused)]

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        arr: [Usize1; n],
    }

    let mut cnt = vec![0; k];
    for &x in &arr {
        cnt[x] += 1;
    }

    let max = *cnt.iter().max().unwrap();
    let mut ans = 0;
    for i in 0..k {
        if cnt[i] + 1 >= max {
            ans += 1;
        }
    }
    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
