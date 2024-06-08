#![allow(unused)]

use std::collections::HashMap;

fn main() {
    let n = read::<usize>();
    let mut arr = readv::<usize>();

    let head = !0;
    let tail = !0 - 1;
    arr.insert(0, head);
    arr.push(tail);

    let mut next = HashMap::new();
    let mut prev = HashMap::new();

    for i in 0..(arr.len() - 1) {
        next.insert(arr[i], arr[i + 1]);
        prev.insert(arr[i + 1], arr[i]);
    }

    let q = read::<usize>();
    for _ in 0..q {
        let ask = readv::<usize>();
        if ask[0] == 1 {
            let (x, y) = (ask[1], ask[2]);
            // p(x) x n(x)
            // p(x) x y n(x)
            let n = next[&x];
            next.insert(x, y);
            next.insert(y, n);
            prev.insert(y, x);
            prev.insert(n, y);
        } else {
            // p(x) x n(x)
            // p(x) n(x)
            let x = ask[1];
            let (p, n) = (prev[&x], next[&x]);
            next.insert(p, n);
            prev.insert(n, p);
        }
    }

    let mut ans = vec![];
    let mut x = next[&head];
    while x != tail {
        ans.push(x);
        x = next[&x];
    }
    println!("{}", join(&ans, " "));
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
    read::<String>().chars().collect::<_>()
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
