#![allow(unused)]

use std::cmp::Reverse;

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let arr = readv::<usize>();
    let arr = mapv(&arr, |&x| x - 1);

    let mut que = std::collections::BinaryHeap::new();
    let mut cnt = vec![0; n];
    let mut ans = vec![];

    for i in 0..m {
        cnt[arr[i]] += 1;
        que.push((cnt[arr[i]], Reverse(arr[i])));
        if let Some(&(_, Reverse(x))) = que.peek() {
            ans.push(x + 1);
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
