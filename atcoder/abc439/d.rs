#![allow(unused)]

use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        arr: [usize; n],
    }

    let mut ans = 0;

    // i, k is at the left of j
    let mut pref = HashMap::new();
    for j in 0..n {
        if j >= 2 {
            if arr[j] % 5 == 0 {
                let cnt1 = pref.get(&(arr[j] / 5 * 7)).unwrap_or(&0);
                let cnt2 = pref.get(&(arr[j] / 5 * 3)).unwrap_or(&0);
                ans += cnt1 * cnt2;
            }
        }
        *pref.entry(arr[j]).or_insert(0) += 1 as i64;
    }

    // i, k is at the right of j
    let mut suff = HashMap::new();
    for j in (0..n).rev() {
        if j < n - 2 {
            if arr[j] % 5 == 0 {
                let cnt1 = suff.get(&(arr[j] / 5 * 7)).unwrap_or(&0);
                let cnt2 = suff.get(&(arr[j] / 5 * 3)).unwrap_or(&0);
                ans += cnt1 * cnt2;
            }
        }
        *suff.entry(arr[j]).or_insert(0) += 1 as i64;
    }

    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
