#![allow(unused)]

use std::collections::BTreeSet;

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        n: usize,
        arr: [i64; n],
    }

    let mut set = BTreeSet::new();
    for i in 0..n {
        set.insert((arr[i], i));
    }

    let mut pos = 0;
    let mut ans = 0;
    let inf = 10i64.pow(18);

    for _ in 0..n {
        let (pos_l, idx_l) = *set.range(..=(pos, n)).last().unwrap_or(&(-inf, 0));
        let (pos_r, idx_r) = *set.range((pos, 0)..).next().unwrap_or(&(inf, 0));

        if pos_l.abs_diff(pos) <= pos_r.abs_diff(pos) {
            set.remove(&(pos_l, idx_l));
            ans += pos_l.abs_diff(pos);
            pos = pos_l;
        } else {
            set.remove(&(pos_r, idx_r));
            ans += pos_r.abs_diff(pos);
            pos = pos_r;
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
