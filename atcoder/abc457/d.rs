#![allow(unused)]

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: u64,
        arr: [u64; n],
    }

    // Is "number of op to make the min(A) = m" <= k
    let ok = |m: u64| -> bool {
        let mut cnt = 0;
        for i in 0..n {
            if arr[i] < m {
                cnt += (m - arr[i] + i as u64) / (i as u64 + 1);
            }
        }

        cnt <= k
    };

    // for i in 0..10 {
    //     println!("{}: {}", i, ok(i));
    // }

    // 1 1 1 0 0 0
    let mut lb = 0;
    let mut ub = 2 * 10u64.pow(18) + 10;
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
