#![allow(unused)]

use std::collections::BTreeSet;

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        arr: [usize; n],
    }

    // 0 1 2 3 4 5 6 7 8
    // 1 4 1 4 2 1 3 5 6
    //   ...........
    //   p

    // max(A[..p]) <= min(A[p..p+k])
    // max(A[p..p+k]) <= min(A[p+k..])

    let mut max_left = vec![0; n];
    max_left[0] = arr[0];
    for i in 1..n {
        max_left[i] = max_left[i - 1].max(arr[i]);
    }

    let mut min_right = vec![0; n];
    min_right[n - 1] = arr[n - 1];
    for i in (0..(n - 1)).rev() {
        min_right[i] = min_right[i + 1].min(arr[i]);
    }

    let mut pref = vec![false; n];
    pref[0] = true;
    for i in 1..n {
        pref[i] = pref[i - 1] && (arr[i] >= arr[i - 1]);
    }

    let mut suff = vec![false; n];
    suff[n - 1] = true;
    for i in (0..(n - 1)).rev() {
        suff[i] = suff[i + 1] && (arr[i + 1] >= arr[i]);
    }

    // sliding window of k
    let mut set = BTreeSet::new();
    for i in 0..n {
        // insert arr[i]
        set.insert((arr[i], i));

        // remove arr[i - k]
        if i >= k {
            set.remove(&(arr[i - k], i - k));
        }

        // check, window = arr[i-k+1..=i]
        if i >= k - 1 {
            let (win_min, _) = *set.iter().next().unwrap();
            let (win_max, _) = *set.iter().last().unwrap();
            let p = i + 1 - k;

            let mut ok = true;
            if p >= 1 {
                ok &= max_left[p - 1] <= win_min;
                ok &= pref[p - 1];
            }
            if p + k < n {
                ok &= win_max <= min_right[p + k];
                ok &= suff[p + k];
            }

            if ok {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
