#![allow(unused)]

use proconio::input;
use std::cmp::Reverse;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut arr_a: [i64; n],
        arr_b: [i64; m],
    }

    let mut set = BTreeSet::new();
    for i in 0..m {
        set.insert((arr_b[i], i));
    }

    arr_a.sort();

    let mut ans = 0;
    for i in 0..n {
        if let Some(&(b, j)) = set.range(..=(2 * arr_a[i], !0)).next() {
            set.remove(&(b, j));
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
