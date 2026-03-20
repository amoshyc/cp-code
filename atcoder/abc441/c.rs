#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: i64,
        mut arr: [i64; n],
    }

    arr.sort();
    arr.reverse();

    let ok = |m: usize| -> bool {
        // drink largest m cups and largest n - k of them is water
        let sake = arr[n - k..m].iter().sum::<i64>();
        sake >= x
    };

    // 0 0 0 1 1 1
    let mut lb = n - k;
    let mut ub = n;
    if !ok(ub) {
        println!("-1");
        return;
    }

    while ub - lb > 1 {
        let m = (lb + ub) / 2;
        if ok(m) {
            ub = m;
        } else {
            lb = m;
        }
    }
    println!("{ub}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
