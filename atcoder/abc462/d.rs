#![allow(unused)]

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        n: usize,
        d: usize,
        st: [(usize, usize); n],
    }

    // arr[i] = number of people that has start time <= i and end time >= i + d
    // For person that is s..=t,
    // if t >= s + d, then arr[s], arr[s + 1], ..., arr[t - d] has to +1

    let mut diff = vec![0; 1_000_000 + 5];
    for (i, &(s, t)) in st.iter().enumerate() {
        if t >= s + d {
            diff[s] += 1;
            diff[t - d + 1] -= 1;
        }
    }

    let mut cnt = vec![0; 1_000_000 + 1];
    cnt[0] = diff[0];
    for i in 1..=1_000_000 {
        cnt[i] = cnt[i - 1] + diff[i];
    }

    let mut ans = 0;
    for i in 0..=1_000_000 {
        if cnt[i] >= 2 {
            ans += cnt[i] as i64 * (cnt[i] as i64 - 1) / 2;
        }
    }

    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
