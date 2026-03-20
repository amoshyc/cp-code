#![allow(unused)]

use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        h: [i64; n],
    }

    let argmax = sliding_arg(n, k, |i, p| h[i] >= h[p]);
    let argmin = sliding_arg(n, k, |i, p| h[i] <= h[p]);

    let ans = (k - 1..n)
        .map(|r| h[argmax[r]] - h[argmin[r]])
        .max()
        .unwrap();
    println!("{ans}");
}

// pop_cond(i, p): do we pop p when inserting i?
// sliding_argmax: |i, p| h[i] >= h[p]
// sliding_argmin: |i, p| h[i] <= h[p]
// res[i] = p <-> arr[p] is the minimum/maximum of res[i - k + 1..=i]
fn sliding_arg<F>(n: usize, k: usize, pop_cond: F) -> Vec<usize>
where
    F: Fn(usize, usize) -> bool,
{
    let mut deq = VecDeque::<usize>::new();
    let mut res = vec![usize::MAX; n];
    for i in 0..n {
        // insert arr[i]
        while let Some(&p) = deq.back() {
            if pop_cond(i, p) {
                deq.pop_back();
            } else {
                break;
            }
        }
        deq.push_back(i);
        // remove arr[i - k]
        if *deq.front().unwrap() + k == i {
            deq.pop_front();
        }
        // check
        res[i] = *deq.front().unwrap();
    }
    res
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
