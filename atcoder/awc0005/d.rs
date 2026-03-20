#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        arr: [i64; n],
    }

    // Can the smallest load be m or larger?
    // = If we split the arr using m, is the number of segments <= k ?
    let ok = |m: i64| -> bool {
        let mut cnt = 1;
        let mut sum = 0;
        for i in 0..n {
            if sum + arr[i] <= m {
                sum += arr[i];
            } else {
                cnt += 1;
                sum = 0;
            }
        }
        cnt <= k
    };

    // for i in 1..=10 {
    //     println!("{}: {}", i, ok(i));
    // }

    // 0 0 0 1 1 1
    let mut lb = *arr.iter().min().unwrap();
    let mut ub = arr.iter().sum::<i64>();
    if ok(lb) {
        println!("{}", lb);
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
    println!("{}", ub);
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
