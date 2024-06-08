#![allow(unused)]

use std::collections::{HashSet, VecDeque};

fn main() {
    let inp = readv::<usize>();
    let (h, w) = (inp[0], inp[1]);
    let mut arr = vec![];
    for _ in 0..h {
        arr.push(reads());
    }

    let mut is_adj = vec![vec![false; w]; h];
    for r in 0..h {
        for c in 0..w {
            for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let nr = r.checked_add_signed(dr).unwrap_or(h);
                let nc = c.checked_add_signed(dc).unwrap_or(w);
                if nr < h && nc < w && arr[nr][nc] == '#' {
                    is_adj[r][c] = true;
                }
            }
        }
    }

    let mut ans = 1;
    let mut vis = vec![vec![false; w]; h];
    for root_r in 0..h {
        for root_c in 0..w {
            if arr[root_r][root_c] == '#' {
                continue;
            }
            if vis[root_r][root_c] {
                continue;
            }
            if is_adj[root_r][root_c] {
                continue;
            }
            let mut que = VecDeque::new();
            let mut node = HashSet::new();

            que.push_back((root_r, root_c));
            node.insert((root_r, root_c));

            while let Some((r, c)) = que.pop_front() {
                if is_adj[r][c] {
                    continue;
                }
                for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let nr = r.checked_add_signed(dr).unwrap_or(h);
                    let nc = c.checked_add_signed(dc).unwrap_or(w);
                    if nr < h && nc < w && !node.contains(&(nr, nc)) {
                        node.insert((nr, nc));
                        que.push_back((nr, nc));
                    }
                }
            }

            ans = ans.max(node.len());
            for (r, c) in node {
                vis[r][c] = true;
            }
        }
    }

    println!("{}", ans);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn readv<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_ascii_whitespace()
        .map(|t| t.parse().ok().unwrap())
        .collect()
}

fn reads() -> Vec<char> {
    read::<String>().chars().collect::<Vec<char>>()
}

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
