#![allow(unused)]

use std::collections::BTreeMap;
use std::collections::BTreeSet;

fn main() {
    let inp = readv::<usize>();
    let (n, q) = (inp[0], inp[1]);
    let mut arr = readv::<usize>();

    let mut missing: BTreeSet<usize> = (0..=n).collect();
    let mut cnt = BTreeMap::new();
    for &a in arr.iter() {
        let entry = cnt.entry(a).or_insert(0);
        *entry += 1;
        if *entry == 1 {
            missing.remove(&a);
        }
    }

    let mut ans = vec![];
    for _ in 0..q {
        let mut inp = readv::<usize>();
        let (i, x) = (inp[0] - 1, inp[1]);

        let entry = cnt.entry(arr[i]).or_insert(0);
        *entry -= 1;
        if *entry == 0 {
            missing.insert(arr[i]);
        }

        arr[i] = x;

        let entry = cnt.entry(x).or_insert(0);
        *entry += 1;
        if *entry == 1 {
            missing.remove(&x);
        }

        ans.push(*missing.first().unwrap());
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
