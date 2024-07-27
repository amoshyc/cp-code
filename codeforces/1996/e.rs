#![allow(unused)]

use std::collections::HashMap;

fn solve() {
    let m = 10i64.pow(9) + 7;
    let s = reads();
    let n = s.len();

    let f = mapv(&s, |&c| if c == '0' { -1 } else { 1 } as i64);
    let p = build(&f);
    // s[x..=y] is valid <-> p[y] = p[x - 1] or p[y] = 0

    let mut pos = HashMap::new();
    pos.entry(&0).or_insert(vec![]).push(-1); // To deal with the case p[y] = 0
    for i in 0..n {
        pos.entry(&p[i]).or_insert(vec![]).push(i as i64);
    }

    let mut ans = 0;
    for (_, v) in pos.iter() {
        // v = [..., x, ..., y, ...]
        // -> p[x] = p[y]
        // -> s[x + 1..=y] is valid
        // -> contribute (n - 1 - y + 1) * (x + 2) to answer
        if v.len() < 2 {
            continue;
        }
        // sweep line over v
        let mut pref_sum = 0;
        for i in v {
            ans += (n as i64 - i) * pref_sum % m;
            ans %= m;
            pref_sum += i + 2;
            pref_sum %= m;
        }
    }

    println!("{}", ans);
}

fn main() {
    let tc = read::<usize>();
    for _ in 0..tc {
        solve();
    }
}

fn build<T>(arr: &[T]) -> Vec<T>
where
    T: Copy + std::ops::Add<Output = T>,
{
    let mut pref = vec![];
    pref.push(arr[0]);
    for i in 1..arr.len() {
        pref.push(pref[i - 1] + arr[i]);
    }
    pref
}

// i..j
fn query<T>(pref: &[T], i: usize, j: usize) -> T
where
    T: Default + Copy + std::ops::Sub<Output = T>,
{
    if i == j {
        return T::default();
    }
    let mut res = pref[j - 1];
    if i > 0 {
        res = res - pref[i - 1];
    }
    res
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
