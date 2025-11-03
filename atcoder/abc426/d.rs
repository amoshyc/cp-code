#![allow(unused)]

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        tc: usize,
        ask: [(usize, Chars); tc],
    }

    let mut ans = vec![];
    for (n, s) in ask {
        let t = s
            .iter()
            .map(|c| if *c == '0' { '1' } else { '0' })
            .collect();
        let ans1 = solve(s);
        let ans2 = solve(t);
        ans.push(ans1.min(ans2));
    }
    println!("{}", join(&ans, "\n"));
}

// To make s all 0
fn solve(s: Vec<char>) -> i64 {
    // 0 1 000 1 0 11 0
    // <--     ------->
    // 1+3     3 + 4
    // Each is "(number of 1) + (number of 0) * 2"

    let arr = rle(&s);
    let m = arr.len();

    if m == 1 {
        return if s[0] == '0' { 0 } else { s.len() as i64 };
    }

    let mut f0 = vec![0; m];
    let mut f1 = vec![0; m];
    for i in 0..m {
        if arr[i].0 == '0' {
            f0[i] = arr[i].1 as i64;
        } else {
            f1[i] = arr[i].1 as i64;
        }
    }
    let pref0 = build(&f0);
    let pref1 = build(&f1);

    let mut min = i64::MAX;
    for i in 0..m {
        if arr[i].0 == '0' {
            let left = query(&pref1, 0, i) + query(&pref0, 0, i) * 2;
            let right = query(&pref1, i + 1, m) + query(&pref0, i + 1, m) * 2;
            min = min.min(left + right);
        }
    }

    min
}

fn rle<T: Copy + PartialEq>(arr: &Vec<T>) -> Vec<(T, usize, usize)> {
    let n = arr.len();
    let mut res = vec![];
    let mut i = 0;
    while i < n {
        let mut j = i + 1;
        while j < n && arr[j] == arr[i] {
            j += 1;
        }
        res.push((arr[i], j - i, i));
        i = j;
    }
    res
}

fn build<T>(arr: &[T]) -> Vec<T>
where
    T: Default + Copy + std::ops::Add<Output = T>,
{
    let mut pref = vec![T::default(); arr.len()];
    pref[0] = arr[0];
    for i in 1..arr.len() {
        pref[i] = pref[i - 1] + arr[i];
    }
    pref
}

// i..j
fn query<T>(pref: &[T], i: usize, j: usize) -> T
where
    T: Default + Copy + std::ops::Sub<Output = T>,
{
    if i == j {
        T::default()
    } else if i == 0 {
        pref[j - 1]
    } else {
        pref[j - 1] - pref[i - 1]
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
