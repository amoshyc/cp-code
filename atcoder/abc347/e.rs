#![allow(unused)]

use std::collections::HashSet;

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let ask = readv::<usize>();

    let mut set = HashSet::new();
    let mut cnt = vec![0; m];
    let mut inv = vec![vec![]; n + 1];
    for (i, &x) in ask.iter().enumerate() {
        if set.contains(&x) {
            set.remove(&x);
        } else {
            set.insert(x);
        }
        cnt[i] = set.len();
        inv[x].push(i);
    }

    let pref = build(&cnt);

    let mut ans = vec![0; n];
    for i in 0..n {
        for j in (0..inv[i + 1].len()) {
            if j % 2 == 0 {
                let l = inv[i + 1][j];
                let r = if j == inv[i + 1].len() - 1 {
                    m
                } else {
                    inv[i + 1][j + 1]
                };
                ans[i] += query(&pref, l, r);
            }
        }
    }

    println!("{}", join(&ans, " "));
}

fn build<T: Copy + std::ops::Add<Output = T>>(arr: &[T]) -> Vec<T> {
    let mut pref = vec![];
    pref.push(arr[0]);
    for i in 1..arr.len() {
        pref.push(pref[i - 1] + arr[i]);
    }
    pref
}

// i..j
fn query<T: Default + Copy + std::ops::Sub<Output = T>>(pref: &[T], i: usize, j: usize) -> T {
    if i == j {
        return T::default();
    }
    let mut res = pref[j - 1];
    if i > 0 {
        res = res - pref[i - 1];
    }
    res
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
