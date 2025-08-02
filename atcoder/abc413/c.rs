#![allow(unused)]

use std::collections::VecDeque;

fn main() {
    let q = read::<usize>();
    let mut que = VecDeque::new();
    let mut ans = vec![];
    for _ in 0..q {
        let ask = readv::<i64>();
        if ask[0] == 1 {
            let (c, x) = (ask[1], ask[2]);
            que.push_back((c, x));
        } else {
            let mut k = ask[1];
            let mut s = 0;
            while k > 0 {
                let (c, x) = que.pop_front().unwrap();
                let n = c.min(k);
                s += n * x;
                k -= n;
                if k == 0 && n != c {
                    que.push_front((c - n, x));
                }
            }
            ans.push(s);
        }
    }
    println!("{}", join(&ans, "\n"));
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    s.trim().parse().ok().unwrap()
}

fn readv<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_ascii_whitespace()
        .map(|t| t.parse().ok().unwrap())
        .collect()
}

fn reads() -> Vec<char> {
    read::<String>().chars().collect()
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
