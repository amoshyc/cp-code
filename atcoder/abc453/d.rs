#![allow(unused)]

use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        mut arr: [Chars; h],
    }

    let (mut sr, mut sc) = (0, 0);
    let (mut gr, mut gc) = (0, 0);
    for r in 0..h {
        for c in 0..w {
            if arr[r][c] == 'S' {
                (sr, sc) = (r, c);
            }
            if arr[r][c] == 'G' {
                (gr, gc) = (r, c);
            }
        }
    }
    arr[sr][sc] = '.';
    arr[gr][gc] = '.';

    let inf = 8 * h * w;
    let mut par = vec![vec![vec![(h, w, 0); w]; h]; 4];
    let mut dis = vec![vec![vec![inf; w]; h]; 4];
    let mut que = VecDeque::new();

    for d in 0..4 {
        dis[d][sr][sc] = 0;
        que.push_back((d, sr, sc));
    }

    while let Some((d, r, c)) = que.pop_front() {
        if (r, c) == (gr, gc) {
            break;
        }

        let mut adj = vec![(0, 0, 1), (1, 0, -1), (2, 1, 0), (3, -1, 0)];
        if arr[r][c] == '.' {
            // do nothing
        } else if arr[r][c] == 'o' {
            adj = vec![adj[d]];
        } else if arr[r][c] == 'x' {
            adj.remove(d);
        }

        for &(nd, dr, dc) in &adj {
            let nr = r.checked_add_signed(dr).unwrap_or(h);
            let nc = c.checked_add_signed(dc).unwrap_or(w);
            if nr >= h || nc >= w || arr[nr][nc] == '#' {
                continue;
            }
            if dis[nd][nr][nc] == inf {
                dis[nd][nr][nc] = dis[d][r][c] + 1;
                par[nd][nr][nc] = (d, r, c);
                que.push_back((nd, nr, nc));
            }
        }
    }

    let mut min = inf;
    let (mut rr, mut cc, mut dd) = (h, w, 0);
    for d in 0..4 {
        if dis[d][gr][gc] < min {
            min = dis[d][gr][gc];
            (rr, cc, dd) = (gr, gc, d);
        }
    }

    if min == inf {
        println!("No");
        return;
    }

    let mut dirs = vec![];
    while (rr, cc) != (sr, sc) {
        dirs.push(["R", "L", "D", "U"][dd]);
        (dd, rr, cc) = par[dd][rr][cc];
    }
    dirs.reverse();
    println!("Yes");
    println!("{}", join(&dirs, ""));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
