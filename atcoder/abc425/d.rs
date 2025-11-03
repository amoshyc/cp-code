#![allow(unused)]

use std::collections::VecDeque;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut arr: [Chars; h],
    }

    let neighbors = |r: usize, c: usize| -> Vec<(usize, usize)> {
        let mut res = vec![];
        for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nr = r.checked_add_signed(dr).unwrap_or(h);
            let nc = c.checked_add_signed(dc).unwrap_or(w);
            if nr < h && nc < w {
                res.push((nr, nc));
            }
        }
        res
    };

    let mut cnt = vec![vec![0; w]; h];
    let mut que = VecDeque::new();
    for r in 0..h {
        for c in 0..w {
            if arr[r][c] == '#' {
                que.push_back((r, c));
            }
        }
    }

    while que.len() > 0 {
        // flip all verts in que
        for &(r, c) in &que {
            arr[r][c] = '#';
            cnt[r][c] = h * w;
            for (nr, nc) in neighbors(r, c) {
                cnt[nr][nc] += 1;
            }
        }

        // Inspect their neighbors
        let mut nxt = VecDeque::new();
        for &(r, c) in &que {
            for (nr, nc) in neighbors(r, c) {
                if cnt[nr][nc] == 1 {
                    nxt.push_back((nr, nc));
                }
            }
        }

        que = nxt;
    }

    let mut ans = 0;
    for r in 0..h {
        for c in 0..w {
            if arr[r][c] == '#' {
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
