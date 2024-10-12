#![allow(unused)]

use std::usize;

fn main() {
    let n = read::<usize>();
    let mut l = i64::MAX;
    let mut r = i64::MAX;
    let mut ans = 0;
    for _ in 0..n {
        let inp = readv::<String>();
        let k = inp[0].parse::<i64>().unwrap();
        let h = inp[1].chars().next().unwrap();

        if h == 'L' {
            if l != i64::MAX {
                ans += (k - l).abs();
            }
            l = k;
        } else {
            if r != i64::MAX {
                ans += (k - r).abs();
            }
            r = k;
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
