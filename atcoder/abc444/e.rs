#![allow(unused)]

use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        d: i64,
        arr: [i64; n],
    }

    // monotonic
    // Two pointers with multiset

    let check = |q: i64, s: &BTreeSet<(i64, usize)>| -> bool {
        if let Some((x, _)) = s.range((q, 0)..).next() {
            if x - q < d {
                return false;
            }
        }
        if let Some((x, _)) = s.range(..(q, !0)).last() {
            if q - x < d {
                return false;
            }
        }
        true
    };

    let mut ans = 0;
    let mut j = 0;
    let mut set = BTreeSet::new();
    for i in 0..n {
        while j < n && check(arr[j], &set) {
            set.insert((arr[j], j));
            j += 1;
        }

        ans += (j - i) as i64;

        set.remove(&(arr[i], i));
    }

    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
