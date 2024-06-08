#![allow(unused)]

use std::collections::HashMap;

fn main() {
    let inp = readv::<usize>();
    let (n, t) = (inp[0], inp[1]);

    let mut score = vec![0; n];
    let mut cnt = HashMap::new();
    cnt.insert(0, n);

    let mut ans = vec![];
    for _ in 0..t {
        let inp = readv::<usize>();
        let (a, b) = (inp[0] - 1, inp[1]);

        cnt.insert(score[a], cnt[&score[a]] - 1);
        if cnt[&score[a]] == 0 {
            cnt.remove(&score[a]);
        }
        score[a] += b;
        *cnt.entry(score[a]).or_insert(0) += 1;

        ans.push(cnt.len());
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
