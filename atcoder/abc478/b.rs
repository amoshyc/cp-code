#![allow(unused)]

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        n: usize,
        v: usize,
        arr: [usize; n],
    }

    let mut ans = 0;
    for a in 0..n {
        for b in (a + 1)..n {
            for c in (b + 1)..n {
                if a + b + c + 3 <= v {
                    ans = ans.max(arr[a] + arr[b] + arr[c]);
                }
            }
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
