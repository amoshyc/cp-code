#![allow(unused)]

use std::collections::HashSet;

use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        q: usize,
        arr: [i64; n],
    }

    let pref = build(&arr);
    let mut offset = 0;
    let mut res = vec![];

    for _ in 0..q {
        input! {
            cmd: usize
        };
        if cmd == 1 {
            input! {c: usize};
            offset += c;
        } else {
            input! {l: Usize1, r: Usize1};
            let l = (l + offset) % n;
            let r = (r + offset) % n;
            let ans = if l <= r {
                query(&pref, l, r + 1)
            } else {
                query(&pref, 0, r + 1) + query(&pref, l, n)
            };

            res.push(ans);
        }
    }

    println!("{}", join(&res, "\n"));
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
