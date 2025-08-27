#![allow(unused)]

use std::collections::VecDeque;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut arr: [Chars; h]
    }

    let (mut sr, mut sc) = (0, 0);
    let (mut gr, mut gc) = (0, 0);
    for r in 0..h {
        for c in 0..w {
            if arr[r][c] == 'S' {
                (sr, sc) = (r, c);
                arr[r][c] = '.';
            } else if arr[r][c] == 'G' {
                (gr, gc) = (r, c);
                arr[r][c] = '.';
            }
        }
    }

    let inf = h * w + 10;
    let mut dis = vec![vec![vec![inf, inf]; w]; h];
    let mut que = VecDeque::new();
    dis[sr][sc][0] = 0;
    que.push_back((sr, sc, 0));
    while let Some((r, c, k)) = que.pop_front() {
        for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nr = r.checked_add_signed(dr).unwrap_or(h);
            let nc = c.checked_add_signed(dc).unwrap_or(w);
            if nr >= h || nc >= w || arr[nr][nc] == '#' {
                continue;
            }
            let nk = if arr[nr][nc] == '?' { k ^ 1 } else { k };
            let ok1 = nk == 0 && arr[nr][nc] == 'o';
            let ok2 = nk == 1 && arr[nr][nc] == 'x';
            let ok3 = arr[nr][nc] == '.';
            let ok4 = arr[nr][nc] == '?';
            if ok1 || ok2 || ok3 || ok4 {
                if dis[r][c][k] + 1 < dis[nr][nc][nk] {
                    dis[nr][nc][nk] = dis[r][c][k] + 1;
                    que.push_back((nr, nc, nk));
                }
            }
        }
    }

    let ans = dis[gr][gc][0].min(dis[gr][gc][1]);
    if ans == inf {
        println!("-1");
    } else {
        println!("{ans}");
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
