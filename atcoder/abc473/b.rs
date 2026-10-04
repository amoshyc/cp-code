#![allow(unused)]

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        n: usize,
        arr: [usize; n],
    }

    let mut cnt = vec![0; 101];
    for &x in &arr {
        cnt[x] += 1;
    }

    let mut ans = 0;
    for i in 1..=100 {
        if cnt[i] % 2 == 1 {
            ans += i;
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
