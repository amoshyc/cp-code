#![allow(unused)]

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: i64,
        x: i64,
        y: i64,
        mut arr_a: [i64; n],
        mut arr_b: [i64; m],
    }

    arr_a.sort();
    arr_b.sort();
    let pref_a = build(&arr_a);
    let pref_b = build(&arr_b);

    let need = arr_b.iter().map(|&z| (z + k - 1) / k).collect::<Vec<_>>();
    let pref_need = build(&need);

    // Enumerate the number of drinks to buy
    let mut ans = 0;
    for cnt_b in 0..=m {
        // we have y k-dollars
        // need to spend pref_need[cnt_b - 1] k-dollars
        // the total price is pref_b[cnt_b - 1] dollars
        if cnt_b > 0 && pref_need[cnt_b - 1] > y {
            continue;
        }
        let change = if cnt_b > 0 {
            y * k - pref_b[cnt_b - 1]
        } else {
            y * k
        };

        // find cnt_a
        let change = x + change;
        let cnt_a = pref_a.partition_point(|&z| z <= change);

        ans = ans.max(cnt_b + cnt_a);
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
