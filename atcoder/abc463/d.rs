#![allow(unused)]

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        mut arr: [(usize, usize); n],
    }

    // ok(m) = can i pick k clothers while the minimum distance in between >= m
    //       = if i pick the clothes as much as possible, will the number of clothes >= k

    // For a pick that maximize the number of clothers,
    // either the left-most cloth or the right-most cloth will be in it.

    arr.sort_by_key(|&(l, r)| (r, l));

    let mut rev = arr.clone();
    rev.sort_by_key(|&(l, r)| (l, r));

    let ok = |m: usize| -> bool {
        // left to right
        let mut cnt1 = 1;
        let mut r = arr[0].1;
        for i in 1..n {
            if arr[i].0 < r + m {
                continue;
            } else {
                cnt1 += 1;
                r = arr[i].1;
            }
        }

        // right to left
        let mut cnt2 = 1;
        let mut l = rev[n - 1].0;
        for i in (0..(n - 1)).rev() {
            if rev[i].1 + m > l {
                continue;
            } else {
                cnt2 += 1;
                l = rev[i].0;
            }
        }

        cnt1.max(cnt2) >= k
    };

    //     0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0
    //  A:   o---------------------o
    //  B:     o---------o
    //  C:           o-------o
    //  D:                   o-------o
    //  E:                     o---------------o
    //  F:                               o---------o

    // for i in 1..=10 {
    //     println!("{}: {}", i, ok(i));
    // }

    // 1 1 1 0 0 0
    let mut lb = 1;
    let mut ub = 1_000_000_000;
    if !ok(lb) {
        println!("-1");
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
    println!("{}", lb);
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
