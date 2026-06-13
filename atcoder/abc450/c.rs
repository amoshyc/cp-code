#![allow(unused)]

use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut ans = 0;
    let mut vis = vec![vec![false; w]; h];

    for sr in 0..h {
        for sc in 0..w {
            if vis[sr][sc] || s[sr][sc] == '#' {
                continue;
            }

            let mut que = VecDeque::new();
            let mut any = false;

            que.push_back((sr, sc));
            vis[sr][sc] = true;

            while let Some((r, c)) = que.pop_front() {
                any |= (r == 0) || (c == 0) || (r == h - 1) || (c == w - 1);
                for (dr, dc) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                    let nr = r.checked_add_signed(dr).unwrap_or(h);
                    let nc = c.checked_add_signed(dc).unwrap_or(w);
                    if nr < h && nc < w && s[nr][nc] == '.' && !vis[nr][nc] {
                        vis[nr][nc] = true;
                        que.push_back((nr, nc));
                    }
                }
            }

            if !any {
                ans += 1;
            }
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
