#![allow(unused)]

use std::cmp::Reverse;

fn main() {
    let n = read::<usize>();
    let mut town = vec![];
    for _ in 0..n {
        let inp = readv::<i64>();
        town.push((inp[0], inp[1]));
    }
    town.sort_by_key(|&(a, b)| Reverse(2 * a + b));

    let mut aoki = town.iter().map(|&(a, b)| a).sum::<i64>();
    let mut taka = 0;
    for (i, &(a, b)) in town.iter().enumerate() {
        aoki -= a;
        taka += a + b;
        if taka > aoki {
            println!("{}", i + 1);
            return;
        }
    }
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

fn mapv<T, S>(arr: &Vec<T>, f: fn(&T) -> S) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
