#![allow(unused)]

use std::cmp::Reverse;

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);

    let prices = readv::<i64>();
    let l = readv::<i64>();
    let d = readv::<i64>();

    let mut p = std::collections::BTreeSet::new();
    for i in 0..n {
        p.insert((prices[i], i));
    }

    let mut c = vec![];
    for i in 0..m {
        c.push((l[i], d[i]));
    }
    c.sort_by_key(|&(l, d)| (Reverse(d), Reverse(l)));

    let mut ans = 0;
    for &(l, d) in c.iter() {
        if let Some(&(x, i)) = p.range((l, 0)..).next() {
            ans += x;
            ans -= d;
            p.remove(&(x, i));
        }
    }

    for &(p, i) in p.iter() {
        ans += p;
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
