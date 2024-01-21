#![allow(unused)]

use std::collections::BTreeSet;

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut events = BTreeSet::new();
    for i in 0..m {
        let inp = readv::<i64>();
        let (t, w, s) = (inp[0], inp[1], inp[2]);
        events.insert((t, 1, w, s));
    }

    let mut cnt = vec![0; n];
    let mut row = BTreeSet::from_iter(0..n);
    while let Some((t, k, w, s)) = events.pop_first() {
        if k == 1 {
            if let Some(&p) = row.first() {
                cnt[p] += w;
                row.remove(&p);
                events.insert((t + s, 0, p as i64, 0));
            }
        } else {
            row.insert(w as usize);
        }
    }

    println!("{}", join(&cnt, "\n"));
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
