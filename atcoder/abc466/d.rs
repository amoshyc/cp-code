#![allow(unused)]

use std::collections::BTreeSet;
use std::io::Write;

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        arr: [(Usize1, Usize1); m]
    }

    let mut rows = BTreeSet::new();
    let mut cols = BTreeSet::new();
    for (i, &(r, c)) in arr.iter().enumerate() {
        rows.insert((r, i));
        cols.insert((c, i));
    }

    let mut ans = 0;
    for (i, &(r, c)) in arr.iter().enumerate() {
        rows.remove(&(r, i));
        cols.remove(&(c, i));

        let mut survive = true;
        if let Some(_) = rows.range((r, 0)..(r + 1, 0)).next() {
            survive = false;
        }
        if let Some(_) = cols.range((c, 0)..(c + 1, 0)).next() {
            survive = false;
        }

        if survive {
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
