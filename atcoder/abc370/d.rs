#![allow(unused)]

use std::collections::BTreeSet;

fn main() {
    let inp = readv::<usize>();
    let (h, w, q) = (inp[0], inp[1], inp[2]);

    let mut rows = vec![BTreeSet::new(); h];
    let mut cols = vec![BTreeSet::new(); w];
    let mut ans = h * w;
    for r in 0..h {
        for c in 0..w {
            rows[r].insert(c);
            cols[c].insert(r);
        }
    }

    for _ in 0..q {
        let ask = readv::<usize>();
        let (qr, qc) = (ask[0] - 1, ask[1] - 1);

        let mut adjs = vec![];
        if let Some(&r) = cols[qc].range(0..=qr).last() {
            adjs.push((r, qc));
        }
        if let Some(&r) = cols[qc].range(qr..h).next() {
            adjs.push((r, qc));
        }
        if let Some(&c) = rows[qr].range(0..=qc).last() {
            adjs.push((qr, c));
        }
        if let Some(&c) = rows[qr].range(qc..w).next() {
            adjs.push((qr, c));
        }
        adjs.sort();
        adjs.dedup();

        for (r, c) in adjs {
            ans -= 1;
            rows[r].remove(&c);
            cols[c].remove(&r);
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
