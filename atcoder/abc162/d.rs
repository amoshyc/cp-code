#![allow(unused)]

use std::collections::HashMap;

fn main() {
    let n = read::<usize>();
    let s = reads();
    let s = mapv(&s, |&c| match c {
        'R' => 0,
        'G' => 1,
        'B' => 2,
        _ => 3,
    });

    let mut f = vec![vec![0; n]; 3];
    for i in 0..n {
        f[s[i]][i] = 1;
    }
    let pref = vec![build(&f[0]), build(&f[1]), build(&f[2])];

    // j - i != k - j
    // k != 2 * j - i
    let mut ans = 0;
    for j in 0..n {
        for i in 0..j {
            if s[i] == s[j] {
                continue;
            }
            let c = 3 - s[j] - s[i]; // color of k
            let ban = 2 * j - i;
            let mut v = query(&pref[c], j + 1, n);
            if ban < n && s[ban] == c {
                v -= 1;
            }
            ans += v as i64;
        }
    }
    println!("{}", ans);
}

fn build<T: Copy + std::ops::Add<Output = T>>(arr: &[T]) -> Vec<T> {
    let mut pref = vec![];
    pref.push(arr[0]);
    for i in 1..arr.len() {
        pref.push(pref[i - 1] + arr[i]);
    }
    pref
}

// i..j
fn query<T: Default + Copy + std::ops::Sub<Output = T>>(pref: &[T], i: usize, j: usize) -> T {
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
    read::<String>().chars().collect::<_>()
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
