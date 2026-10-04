#![allow(unused)]

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
        mut ops: [(Usize1, Usize1, char); q],
    }

    ops.insert(0, (h - 1, w - 1, 'A'));
    let mut entries = vec![vec![(0, ' '); w]; h];
    for (t, &(r, c, x)) in ops.iter().enumerate() {
        let entry = (t, x);
        if entry > entries[r][c] {
            entries[r][c] = entry;
        }
    }

    let mut dp = vec![vec![(0, ' '); w]; h];
    for r in (0..h).rev() {
        for c in (0..w).rev() {
            dp[r][c] = entries[r][c];
            if r + 1 < h {
                dp[r][c] = dp[r][c].max(dp[r + 1][c]);
            }
            if c + 1 < w {
                dp[r][c] = dp[r][c].max(dp[r][c + 1]);
            }
        }
    }

    let mut res = vec![];
    for r in 0..h {
        let row = dp[r].iter().map(|&(_, x)| x).collect::<Vec<_>>();
        res.push(join(&row, ""));
    }
    println!("{}", join(&res, "\n"));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
