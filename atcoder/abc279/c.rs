use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main() {
    let inp = readv::<usize>();
    let (h, w) = (inp[0], inp[1]);

    let mut a = vec![];
    let mut b = vec![];
    for _ in 0..h {
        a.push(reads());
    }
    for _ in 0..h {
        b.push(reads());
    }

    let mut ha = vec![];
    let mut hb = vec![];
    for c in 0..w {
        let col_a = a.iter().map(|row| row[c]).collect::<Vec<char>>();
        let col_b = b.iter().map(|row| row[c]).collect::<Vec<char>>();

        let mut h = DefaultHasher::new();
        col_a.hash(&mut h);
        ha.push(h.finish());

        let mut h = DefaultHasher::new();
        col_b.hash(&mut h);
        hb.push(h.finish());
    }
    ha.sort();
    hb.sort();

    // println!("{:?}", ha);
    // println!("{:?}", hb);

    println!("{}", if ha == hb { "Yes" } else { "No" });
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