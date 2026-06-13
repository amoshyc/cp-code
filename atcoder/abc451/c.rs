#![allow(unused)]

use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        q: usize,
        asks: [(usize, usize); q],
    }

    let mut set = BTreeSet::new();
    let mut ans = vec![0; q];
    for (i, &(cmd, x)) in asks.iter().enumerate() {
        if cmd == 1 {
            set.insert((x, i));
        } else {
            while let Some(&(y, j)) = set.first() {
                if y <= x {
                    set.remove(&(y, j));
                } else {
                    break;
                }
            }
        }
        ans[i] = set.len();
    }

    println!("{}", join(&ans, "\n"));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
