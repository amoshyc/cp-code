#![allow(unused)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
    }

    let mut ans = vec![vec![0; w]; h];
    for r in 0..h {
        for c in 0..w {
            for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let nr = r.checked_add_signed(dr).unwrap_or(h);
                let nc = c.checked_add_signed(dc).unwrap_or(w);
                if nr < h && nc < w {
                    ans[r][c] += 1;
                }
            }
        }
    }

    for r in 0..h {
        println!("{}", join(&ans[r], " "));
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
