#![allow(unused)]

use std::collections::HashMap;

fn main() {
    let n = read::<usize>();
    let arr = readv::<usize>();

    let mut pos = HashMap::new();
    for i in 0..n {
        pos.entry(arr[i]).or_insert(vec![]).push(i as i64);
    }

    let mut ans = 0;
    for (k, mut ps) in pos {
        let mut cnt = (n * (n + 1) / 2) as i64;
        ps.insert(0, -1);
        ps.push(n as i64);
        for w in ps.windows(2) {
            let l = w[1] - w[0] - 1;
            cnt -= l * (l + 1) / 2;
        }
        ans += cnt;
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
