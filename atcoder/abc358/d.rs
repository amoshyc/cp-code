#![allow(unused)]

use std::collections::BTreeSet;

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut arr_a = readv::<i64>();
    arr_a.sort();

    let arr_b = readv::<i64>();
    let mut set = BTreeSet::new();
    for i in 0..m {
        set.insert((arr_b[i], i));
    }

    let mut ans = 0;
    for i in 0..n {
        if let Some((x, j)) = set.range(..=(arr_a[i], !0)).last().cloned() {
            ans += arr_a[i];
            set.remove(&(x, j));
        }
    }

    if set.len() == 0 {
        println!("{}", ans);
    } else {
        println!("-1");
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
