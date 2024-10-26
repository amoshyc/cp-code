#![allow(unused)]

use std::collections::BTreeSet;

fn main() {
    let n = read::<usize>();
    let mut arr_a = readv::<i64>();
    let arr_b = readv::<i64>();

    let mut set_b = BTreeSet::new();
    for i in 0..(n - 1) {
        set_b.insert((arr_b[i], i));
    }

    let mut f = vec![false; n];
    arr_a.sort();
    arr_a.reverse();
    for i in 0..n {
        if let Some(&(b, j)) = set_b.range((arr_a[i], 0)..).next() {
            f[i] = true;
            set_b.remove(&(b, j));
        }
    }

    let cnt = f.iter().filter(|f| !**f).count();
    if cnt >= 2 {
        println!("-1");
        return;
    }

    for i in 0..n {
        if !f[i] {
            println!("{}", arr_a[i]);
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
