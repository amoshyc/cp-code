#![allow(unused)]

use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: i64,
        m: i64,
        rc: [(i64, i64); m]
    }

    let mut set = HashSet::new();
    for (r, c) in rc {
        let adj = [(r, c), (r + 1, c), (r, c + 1), (r + 1, c + 1)];

        let ok = adj.iter().all(|&(a, b)| !set.contains(&(a, b)));

        if ok {
            for (a, b) in adj {
                set.insert((a, b));
            }
        }
    }

    println!("{}", set.len() / 4);
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
