#![allow(unused)]

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut used = vec![false; m];
    let mut ans = vec![0; n];
    for i in 0..n {
        input! {
            l: usize,
            arr: [Usize1; l],
        }

        for x in arr {
            if !used[x] {
                used[x] = true;
                ans[i] = x + 1;
                break;
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
