#![allow(unused)]

use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        n: usize,
        mut arr_a: [u64; n + 1],
        arr_b: [u64; n],
    }

    let mut ans = 0;
    for i in 0..n {
        let mut x = arr_b[i];
        // i
        let k = x.min(arr_a[i]);
        x -= k;
        arr_a[i] -= k;
        ans += k;
        // i + 1
        let k = x.min(arr_a[i + 1]);
        x -= k;
        arr_a[i + 1] -= k;
        ans += k;
    }

    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
