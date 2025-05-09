#![allow(unused)]

use std::collections::HashSet;

fn main() {
    let n = read::<usize>();
    let arr = readv::<usize>();

    let mut pref = vec![1; n];
    let mut set = HashSet::new();
    set.insert(arr[0]);
    for i in 1..n {
        set.insert(arr[i]);
        pref[i] = set.len();
    }

    let mut suff = vec![1; n];
    let mut set = HashSet::new();
    set.insert(arr[n - 1]);
    for i in (0..(n - 1)).rev() {
        set.insert(arr[i]);
        suff[i] = set.len();
    }

    let mut ans = 0;
    for i in 0..n {
        // arr[..i], arr[i..]
        let mut val = 0;
        if i >= 1 {
            val += pref[i - 1];
        }
        val += suff[i];
        ans = ans.max(val);
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
