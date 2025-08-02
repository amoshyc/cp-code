#![allow(unused)]

use std::collections::VecDeque;

fn main() {
    let inp = readv::<usize>();
    let (h, w) = (inp[0], inp[1]);
    let mut arr = vec![];
    for _ in 0..h {
        arr.push(reads());
    }

    let inv_dir = ['<', '>', '^', 'v'];

    let mut que = VecDeque::new();
    let mut ans = arr.clone();
    for r in 0..h {
        for c in 0..w {
            if arr[r][c] == 'E' {
                que.push_back((r, c));
            }
        }
    }
    while let Some((r, c)) = que.pop_front() {
        for (i, &(dr, dc)) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter().enumerate() {
            let nr = r.checked_add_signed(dr).unwrap_or(h);
            let nc = c.checked_add_signed(dc).unwrap_or(w);
            if nr < h && nc < w && ans[nr][nc] == '.' {
                ans[nr][nc] = inv_dir[i];
                que.push_back((nr, nc));
            }
        }
    }

    let mut res = vec![];
    for r in 0..h {
        res.push(join(&ans[r], ""));
    }
    println!("{}", join(&res, "\n"));
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
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
