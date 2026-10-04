#![allow(unused)]

use std::collections::BTreeSet;

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        n: usize,
        arr: [(usize, usize); n],
        q: usize,
        asks: [usize; q],
    }

    let mut heights = BTreeSet::new();
    let mut events = vec![];
    for (i, &(h, t)) in arr.iter().enumerate() {
        heights.insert((h, i));
        events.push((t, 'l', h));
    }
    for (qid, &t) in asks.iter().enumerate() {
        events.push((t, 'q', qid));
    }
    events.sort();

    let mut ans = vec![0; q];
    for (t, op, x) in events {
        if op == 'l' {
            if let Some(&(h, i)) = heights.range((x, 0)..).next() {
                heights.remove(&(h, i));
            }
        } else {
            ans[x] = heights.last().unwrap().0;
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
