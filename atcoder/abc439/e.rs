#![allow(unused)]

use proconio::input;
use std::cmp::Reverse;

fn main() {
    input! {
        n: usize,
        mut ab: [(usize, usize); n],
    }

    // Tha problem is asking the longest chain that (A[i] < A[i + 1] and B[i] < B[i + 1]).
    // which is the 2D Poset problem that can be solved using LIS or Sweep Line + BIT

    ab.sort_by_key(|&(a, b)| (a, Reverse(b)));
    let bs = ab.iter().map(|&(a, b)| b).collect();
    let dp = longest_increasing_subsequence(&bs, !0);
    let ans = dp.iter().max().unwrap();
    println!("{ans}");
}

fn longest_increasing_subsequence<T>(arr: &Vec<T>, inf: T) -> Vec<usize>
where
    T: Clone + PartialOrd,
{
    // The length of the LIS is dp.iter().max().unwrap()
    // aux[0] is meaningless, so we skip it.
    // weakly: <=, strictly: <
    let n = arr.len();
    let mut aux = vec![inf; n + 1]; // Note the n + 1
    let mut dp = vec![0; n];
    for i in 0..n {
        dp[i] = aux[1..].partition_point(|x| *x < arr[i]) + 1;
        aux[dp[i]] = arr[i].clone();
    }
    dp
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
