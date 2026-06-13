#![allow(unused)]

use proconio::{input, marker::Usize1};
use std::{cmp::Reverse, collections::HashSet};

fn main() {
    input! {
        n: usize,
        k: usize,
        m: usize,
        mut cv: [(Usize1, i64); n],
    }

    cv.sort_by_key(|&(c, v)| (Reverse(v), c));

    let mut ans = 0;
    let mut cnt = 0;
    let mut colors = HashSet::new();
    let mut used = vec![false; n];

    for (i, &(c, v)) in cv.iter().enumerate() {
        if colors.contains(&c) {
            continue;
        } else {
            colors.insert(c);
            ans += v;
            cnt += 1;
            used[i] = true;
            if colors.len() == m {
                break;
            }
        }
    }

    for (i, &(c, v)) in cv.iter().enumerate() {
        if !used[i] && cnt < k {
            cnt += 1;
            ans += v;
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
