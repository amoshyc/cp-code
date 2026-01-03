#![allow(unused)]

use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        lr: [(usize, usize); q],
    }

    let mut ans = vec![];
    let mut segs = BTreeSet::new();
    let mut total = 0; // total length of the union

    for (l, r) in lr {
        let mut new_l = l;
        let mut new_r = r;
        while let Some(&(b, a)) = segs.range((new_l, 0)..).next() {
            if new_r < a {
                break;
            }
            segs.remove(&(b, a));
            total -= b - a + 1;
            new_l = new_l.min(a);
            new_r = new_r.max(b);
        }

        segs.insert((new_r, new_l));
        total += new_r - new_l + 1;

        ans.push(n - total);
    }

    println!("{}", join(&ans, "\n"));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
