#![allow(unused)]

use std::collections::HashSet;

fn main() {
    let inp = readv::<usize>();
    let (n, m, q) = (inp[0], inp[1], inp[2]);

    let mut all = HashSet::new();
    let mut grt = HashSet::new();

    let mut ans = vec![];
    for _ in 0..q {
        let ask = readv::<usize>();
        if ask[0] == 1 {
            grt.insert((ask[1], ask[2]));
        } else if ask[0] == 2 {
            all.insert(ask[1]);
        } else {
            let (x, y) = (ask[1], ask[2]);
            if all.contains(&x) || grt.contains(&(x, y)) {
                ans.push("Yes");
            } else {
                ans.push("No");
            }
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
