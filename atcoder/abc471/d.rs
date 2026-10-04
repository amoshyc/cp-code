#![allow(unused)]

use std::collections::BTreeSet;

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        q: usize,
        v: i64,
    }

    let mut set = BTreeSet::new();
    let mut ans = vec![];

    for id in 0..q {
        input! {
            cmd: usize,
            t: i64,
        }

        if cmd == 1 {
            input! { w: i64 }
            set.insert((w - t, id, t, w)); // as if it is inserted at time 0
        } else {
            if let Some(&(x, id, t0, w)) = set.last() {
                // it is inserted at t0 with w, now it is min(w + (t - t0), v)
                ans.push((w + (t - t0)).min(v));
                set.remove(&(x, id, t0, w));
            } else {
                ans.push(-1);
            }
        }
    }

    println!("{}", join(&ans, "\n"));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
