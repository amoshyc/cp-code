#![allow(unused)]

use std::collections::{BinaryHeap, HashSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: usize,
        mut arr: [i64; n],
    }

    arr.sort();
    arr.reverse();

    let mut ans = vec![];
    let mut vis = HashSet::new();
    let mut que = BinaryHeap::new();

    let mut chosen = vec![0; n];
    chosen[0] = k;
    vis.insert(chosen.clone());
    que.push((arr[0] * (k as i64), chosen));

    while let Some((sum, chosen)) = que.pop() {
        ans.push(sum);
        if ans.len() == x {
            break;
        }

        for j in 0..(n - 1) {
            if chosen[j] > 0 {
                let new_sum = sum - arr[j] + arr[j + 1];
                let mut new_chosen = chosen.clone();
                new_chosen[j] -= 1;
                new_chosen[j + 1] += 1;
                if !vis.contains(&new_chosen) {
                    vis.insert(new_chosen.clone());
                    que.push((new_sum, new_chosen));
                }
            }
        }
    }

    println!("{}", join(&ans, "\n"));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
