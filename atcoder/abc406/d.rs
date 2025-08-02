#![allow(unused)]

use std::collections::HashSet;

fn main() {
    let inp = readv::<usize>();
    let (h, w, n) = (inp[0], inp[1], inp[2]);

    let mut rows = vec![0; h];
    let mut cols = vec![0; w];
    let mut inv_rows = vec![vec![]; h];
    let mut inv_cols = vec![vec![]; w];
    for _ in 0..n {
        let rc = readv::<usize>();
        let (r, c) = (rc[0] - 1, rc[1] - 1);
        rows[r] += 1;
        cols[c] += 1;
        inv_rows[r].push(c);
        inv_cols[c].push(r);
    }

    let q = read::<usize>();
    let mut ans = vec![0; q];
    let mut removed = HashSet::new();
    for i in 0..q {
        let ask = readv::<usize>();
        if ask[0] == 1 {
            let r = ask[1] - 1;
            ans[i] = rows[r];
            for &c in &inv_rows[r] {
                if !removed.contains(&(r, c)) {
                    rows[r] -= 1;
                    cols[c] -= 1;
                    removed.insert((r, c));
                }
            }
            inv_rows[r].clear();
        } else {
            let c = ask[1] - 1;
            ans[i] = cols[c];
            for &r in &inv_cols[c] {
                if !removed.contains(&(r, c)) {
                    rows[r] -= 1;
                    cols[c] -= 1;
                    removed.insert((r, c));
                }
            }
            inv_cols[c].clear();
        }
    }

    println!("{}", join(&ans, "\n"));
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
