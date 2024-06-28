#![allow(unused)]

use std::collections::VecDeque;

fn solve() -> i32 {
    let inp = readv::<usize>();
    let (mut a, mut b, mut c) = (inp[0], inp[1], inp[2]);

    let up = a + b + c - 1;
    let down = a * 2 + b * 1;
    if up != down {
        return -1;
    }

    let mut que = VecDeque::new();
    let mut dep = vec![0];

    if a > 0 {
        que.push_back(dep.len());
        dep.push(1);
        que.push_back(dep.len());
        dep.push(1);
        a -= 1;
    } else if b > 0 {
        que.push_back(dep.len());
        dep.push(1);
        b -= 1
    } else {
        c -= 1;
    }

    let mut ans = 0;
    while let Some(u) = que.pop_front() {
        ans = ans.max(dep[u]);
        if a > 0 {
            que.push_back(dep.len());
            dep.push(dep[u] + 1);
            que.push_back(dep.len());
            dep.push(dep[u] + 1);
            a -= 1;
        } else if b > 0 {
            que.push_back(dep.len());
            dep.push(dep[u] + 1);
            b -= 1;
        } else {
            c -= 1;
        }
    }

    ans
}

fn main() {
    let tc = read::<usize>();
    let mut ans = vec![];
    for _ in 0..tc {
        ans.push(solve());
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
