#![allow(unused)]

use std::collections::BTreeSet;

fn main() {
    let inp = readv::<usize>();
    let (h, w, x) = (inp[0], inp[1], inp[2] as i64);
    let inp = readv::<usize>();
    let (p, q) = (inp[0] - 1, inp[1] - 1);

    let mut arr = vec![];
    for _ in 0..h {
        arr.push(readv::<i64>());
    }

    let mut add = vec![vec![false; w]; h];
    let mut adj = BTreeSet::new();
    let mut ans = arr[p][q];

    add[p][q] = true;
    for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let nr = p.checked_add_signed(dr).unwrap_or(h);
        let nc = q.checked_add_signed(dc).unwrap_or(w);
        if nr < h && nc < w && !add[nr][nc] {
            add[nr][nc] = true;
            adj.insert((arr[nr][nc].saturating_mul(x), (nr, nc)));
        }
    }

    while let Some((v, (r, c))) = adj.range(..(ans, (0, 0))).last().cloned() {
        ans += arr[r][c];
        adj.remove(&(v, (r, c)));
        for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nr = r.checked_add_signed(dr).unwrap_or(h);
            let nc = c.checked_add_signed(dc).unwrap_or(w);
            if nr < h && nc < w && !add[nr][nc] {
                add[nr][nc] = true;
                adj.insert((arr[nr][nc].saturating_mul(x), (nr, nc)));
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
