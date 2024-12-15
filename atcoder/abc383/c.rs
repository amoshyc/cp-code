#![allow(unused)]

use std::collections::VecDeque;

fn main() {
    let inp = readv::<usize>();
    let (h, w, d) = (inp[0], inp[1], inp[2]);

    let inf = 1_000_001;
    let mut arr = vec![];
    let mut que = VecDeque::new();
    let mut dis = vec![vec![inf; w]; h];
    for r in 0..h {
        let inp = reads();
        for c in 0..w {
            if inp[c] == 'H' {
                dis[r][c] = 0;
                que.push_back((r, c));
            }
        }
        arr.push(inp);
    }

    while let Some((r, c)) = que.pop_front() {
        for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nr = r.checked_add_signed(dr).unwrap_or(h);
            let nc = c.checked_add_signed(dc).unwrap_or(w);
            if nr < h && nc < w && arr[nr][nc] == '.' && dis[nr][nc] == inf {
                dis[nr][nc] = dis[r][c] + 1;
                que.push_back((nr, nc));
            }
        }
    }

    let mut ans = 0;
    for r in 0..h {
        for c in 0..w {
            if dis[r][c] <= d {
                ans += 1;
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
    read::<String>().chars().collect()
}

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
