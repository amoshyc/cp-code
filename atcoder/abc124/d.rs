#![allow(unused)]

use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }

    // 001000101001
    //     ------
    //      ^ ^ ^
    let ok = |m: usize| -> bool {
        let mut mark = vec![false; n];
        let mut cnt = 0;
        for i in 0..n {
            // insert s[i]
            if s[i] == '0' {
                if i >= 1 && s[i - 1] == s[i] {
                    // segment continues
                    mark[i - 1] = false;
                    mark[i] = true;
                } else {
                    // new segment
                    mark[i] = true;
                    cnt += 1;
                }
            }
            // remove s[i - m]
            if i >= m && s[i - m] == '0' {
                if mark[i - m] {
                    cnt -= 1;
                }
            }
            // check
            if i >= m - 1 && cnt <= k {
                return true;
            }
        }

        false
    };

    // for i in 1..=n.min(10) {
    //     println!("{}: {}", i, ok(i));
    // }

    // 1 1 1 0 0 0
    //     ^
    let mut lb = 1;
    let mut ub = n;
    if ok(ub) {
        println!("{ub}");
        return;
    }
    while ub - lb > 1 {
        let m = (lb + ub) / 2;
        if ok(m) {
            lb = m;
        } else {
            ub = m;
        }
    }
    println!("{lb}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
