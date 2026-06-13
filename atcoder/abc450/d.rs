#![allow(unused)]

use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        mut arr: [i64; n],
    }

    // increase arr[i] to make it the same scale as max(arr)
    // max arr[i] + qk, arr[i] + qk <= arr[-1]
    let max = *arr.iter().max().unwrap();
    for i in 0..n {
        let q = (max - arr[i]) / k;
        arr[i] += q * k;
    }
    arr.sort();

    let mut ans = arr[n - 1] - arr[0];
    let mut arr = VecDeque::from(arr);
    for _ in 0..(n - 1) {
        // Replace minimum value `min` of arr with `min + k`
        let min = arr.pop_front().unwrap();
        arr.push_back(min + k);
        ans = ans.min(arr.back().unwrap() - arr.front().unwrap());
    }

    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
