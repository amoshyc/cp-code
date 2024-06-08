#![allow(unused)]

use std::collections::VecDeque;

fn main() {
    let inp = readv::<i64>();
    let (n, a, b) = (inp[0], inp[1], inp[2]);
    let arr = readv::<i64>();

    let mut rs = mapv(&arr, |x| x % (a + b));
    rs.sort();
    rs.dedup();

    let mut que = VecDeque::new();
    for r in rs {
        que.push_back(r);
    }

    for _ in 0..que.len() {
        if que[que.len() - 1] - que[0] + 1 <= a {
            println!("Yes");
            return;
        }
        let x = que.pop_front().unwrap();
        que.push_back(x + a + b);
    }

    println!("No");
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
