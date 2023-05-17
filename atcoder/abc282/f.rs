#![allow(unused)]

use std::io::Write;
use std::collections::HashMap;

fn main() {
    let n = read::<usize>();

    let mut pairs = vec![];
    let mut w = 1;
    while w <= n {
        for i in 0..=(n - w) {
            pairs.push((i, i + w)); // l..r
        }
        w = w * 2;
    }

    let mut inv = HashMap::new();
    for (i, &p) in pairs.iter().enumerate() {
        inv.insert(p, i + 1);
    }

    let s = pairs
        .iter()
        .map(|&(l, r)| format!("{} {}", l + 1, r)) // l..=r
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", pairs.len());
    println!("{}", s);
    std::io::stdout().flush();

    let q = read::<usize>();
    for _ in 0..q {
        let inp = readv::<usize>();
        let (l, r) = (inp[0] - 1, inp[1]); // l..r
        let w = 1 << ((r - l) as f64).log2().floor() as usize;
        let x = inv[&(l, l + w)];
        let y = inv[&(r - w, r)];
        println!("{} {}", x, y);
        std::io::stdout().flush();
    }
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
