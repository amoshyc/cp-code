#![allow(unused)]

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        need_a: i64,
        need_b: i64,
        s: Chars,
    }

    let fa = (0..n).map(|i| (s[i] == 'a') as i64).collect::<Vec<_>>();
    let fb = (0..n).map(|i| (s[i] == 'b') as i64).collect::<Vec<_>>();
    let pref_a = build(&fa);
    let pref_b = build(&fb);

    // pref[r] - pref[l - 1] >= need_a, pref[r] - pref[l - 1] < need_b
    // pref[r] >= pref[l - 1] + need_a, pref[r] < need_b + pref[l - 1]
    let mut ans = 0;

    // l = 0
    for r in 0..n {
        if pref_a[r] >= need_a && pref_b[r] < need_b {
            ans += 1;
        }
    }
    // l = 1..n
    for l in 1..n {
        let x = pref_a.partition_point(|&x| x < pref_a[l - 1] + need_a); // x..n
        let y = pref_b.partition_point(|&x| x < need_b + pref_b[l - 1]); // l..y
        let x = x.max(l) as i64;
        let y = y.min(n) as i64;
        ans += (y - x).max(0);
    }

    println!("{ans}");
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
