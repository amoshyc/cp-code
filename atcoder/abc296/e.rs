#![allow(unused)]

use std::collections::VecDeque;
use std::iter::FromIterator;

fn main() {
    let n = read::<usize>();
    let arr = readv::<usize>();
    let nxt = arr.iter().map(|&x| x - 1).collect::<Vec<_>>();

    let mut ind = vec![0; n];
    for &v in nxt.iter() {
        ind[v] += 1;
    }

    let mut ans = n;
    let mut que = VecDeque::from_iter((0..n).filter(|&v| ind[v] == 0));
    while let Some(u) = que.pop_front() {
        ans -= 1;
        let v = nxt[u];
        ind[v] -= 1;
        if ind[v] == 0 {
            que.push_back(v);
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
