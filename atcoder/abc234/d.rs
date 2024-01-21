#![allow(unused)]

use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    let inp = readv::<usize>();
    let (n, k) = (inp[0], inp[1]);
    let p = readv::<usize>();

    let mut ans: Vec<usize> = vec![];
    let mut cands = std::collections::BinaryHeap::new();
    for i in 0..k {
        cands.push(Reverse(p[i]));
    }
    if let Some(Reverse(x)) = cands.peek() {
        ans.push(*x);
    }

    for i in k..n {
        cands.push(Reverse(p[i]));
        cands.pop();
        if let Some(Reverse(x)) = cands.peek() {
            ans.push(*x);
        }
    }

    println!("{}", join(&ans, "\n"));

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
