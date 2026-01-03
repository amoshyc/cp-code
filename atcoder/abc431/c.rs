#![allow(unused)]

use std::collections::BTreeSet;

use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        mut arr_h: [i64; n],
        mut arr_b: [i64; m],
    }

    let mut set = BTreeSet::new();
    for i in 0..m {
        set.insert((arr_b[i], i));
    }

    let mut cnt = 0;
    arr_h.sort();
    arr_h.reverse();
    for i in 0..n {
        if let Some(&(b, j)) = set.range((arr_h[i], 0)..).next() {
            set.remove(&(b, j));
            cnt += 1;
        }
    }

    if cnt >= k {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
