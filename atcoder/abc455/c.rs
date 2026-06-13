#![allow(unused)]

use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        arr: [i64; n],
    }

    let mut cnt = HashMap::new();
    for i in 0..n {
        *cnt.entry(arr[i]).or_insert(0) += 1;
    }

    let mut sums = vec![];
    for (k, v) in cnt.iter() {
        let sum = k * v;
        if sum > 0 {
            sums.push(sum);
        }
    }

    sums.sort();

    let mut ans = arr.iter().sum::<i64>();
    for _ in 0..k {
        if let Some(sum) = sums.pop() {
            ans -= sum;
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
