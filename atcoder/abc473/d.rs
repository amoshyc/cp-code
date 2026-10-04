#![allow(unused)]

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        n: usize,
        k: i64,
    }

    let mut arr = vec![0; n];
    let mut ans = vec![];
    dfs(&mut arr, 0, 0, k, &mut ans);

    println!("{}", join(&ans, "\n"));
}

fn dfs(arr: &mut Vec<i64>, i: usize, sum: i64, k: i64, ans: &mut Vec<String>) {
    if i == arr.len() - 1 {
        if (k - sum) % ((i + 1) as i64) == 0 {
            arr[i] = (k - sum) / ((i + 1) as i64);
            ans.push(join(&arr, " "));
        }
        return;
    }

    // sum + (i + 1) * x <= k
    // x <= (k - sum) / (i + 1)
    let ub = (k - sum) / ((i + 1) as i64);
    for x in 0..=ub {
        arr[i] = x;
        dfs(arr, i + 1, sum + ((i + 1) as i64) * x, k, ans);
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
