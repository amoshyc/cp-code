#![allow(unused)]

use std::collections::VecDeque;

use proconio::{input, marker::Chars, marker::Usize1};

fn main() {
    input! {
        tc: usize,
    }

    let mut res = vec![];
    for _ in 0..tc {
        input! {
            n: usize,
            c0: Usize1,
            arr: [Chars; n],
        }

        let mut suff = vec![vec![false; n]; n];
        for r in 0..n {
            for c in 0..n {
                suff[r][c] = (arr[r][c] == '.') as bool;
            }
        }
        for r in (0..(n - 1)).rev() {
            for c in 0..n {
                suff[r][c] &= suff[r + 1][c];
            }
        }

        let mut vis = vec![vec![false; n]; n];
        let mut que = VecDeque::new();

        suff[n - 1][c0] = true;
        vis[n - 1][c0] = true;
        que.push_back((n - 1, c0));

        while let Some((r, c)) = que.pop_front() {
            if r == 0 {
                continue;
            }

            for dc in [-1, 0, 1] {
                let nc = c.checked_add_signed(dc).unwrap_or(n);
                if nc < n && !vis[r - 1][nc] {
                    let ok1 = arr[r - 1][nc] == '.';
                    let ok2 = arr[r - 1][nc] == '#' && suff[r][nc];
                    if ok1 || ok2 {
                        vis[r - 1][nc] = true;
                        suff[r - 1][nc] = suff[r][nc];
                        que.push_back((r - 1, nc));
                    }
                }
            }
        }

        let r0 = vis[0]
            .iter()
            .map(|&x| if x { '1' } else { '0' })
            .collect::<Vec<_>>();
        res.push(join(&r0, ""));
    }

    println!("{}", join(&res, "\n"));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
