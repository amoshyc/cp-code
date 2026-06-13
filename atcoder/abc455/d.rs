#![allow(unused)]

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        asks: [(Usize1, Usize1); q],
    }

    let mut next = vec![None; 2 * n];
    let mut prev = vec![None; 2 * n];

    // setup
    for i in 0..n {
        next[n + i] = Some(i);
        prev[i] = Some(n + i);
    }

    // simulate
    for (c, p) in asks {
        if let Some(pc) = prev[c] {
            next[pc] = None;
        }
        next[p] = Some(c);
        prev[c] = Some(p);
    }

    // count
    let mut ans = vec![0; n];
    for i in 0..n {
        let mut x = n + i;
        while let Some(y) = next[x] {
            ans[i] += 1;
            x = y;
        }
    }

    println!("{}", join(&ans, " "));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
