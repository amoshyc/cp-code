#![allow(unused)]

use proconio::input;

const MOD: i64 = 998244353;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut arr_a: [i64; n],
        arr_b: [i64; m],
    }

    arr_a.sort();

    let pref = build(&arr_a);

    let mut ans = 0;
    for b in arr_b {
        let i = arr_a.partition_point(|&x| x <= b);
        let v1 = i as i64 * b - query(&pref, 0, i);
        let v2 = query(&pref, i, n) - (n - i) as i64 * b;
        ans += v1 % MOD;
        ans %= MOD;
        ans += v2 % MOD;
        ans %= MOD;
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
