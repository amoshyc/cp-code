#![allow(unused)]

use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut s = vec![];
    for _ in 0..n {
        let inp = reads();
        let indices = (0..m).filter(|&i| inp[i] == 'o');
        s.push(HashSet::<usize>::from_iter(indices));
    }

    let mut ans = 0;
    for x in 0..n {
        for y in (x + 1)..n {
            if s[x].union(&s[y]).count() == m {
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
    read::<String>().chars().collect::<Vec<char>>()
}
