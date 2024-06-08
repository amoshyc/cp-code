#![allow(unused)]

use std::cmp::Reverse;

fn main() {
    let n = read::<usize>();
    let mut x = vec![];
    let mut y = vec![];
    for _ in 0..n {
        let inp = readv::<i64>();
        x.push(inp[0]);
        y.push(inp[1]);
    }

    let mut ans = vec![];
    for i in 0..n {
        let j = (0..n)
            .filter(|j| *j != i)
            .max_by_key(|j| {
                let dx = x[*j] - x[i];
                let dy = y[*j] - y[i];
                (dx * dx + dy * dy, Reverse(*j))
            })
            .unwrap();
        ans.push(j + 1);
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
