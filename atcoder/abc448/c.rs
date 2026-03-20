#![allow(unused)]

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        arr: [usize; n],
    }

    let mut indices = (0..n).collect::<Vec<_>>();
    indices.sort_by_key(|&i| arr[i]);

    let mut ans = vec![];
    for _ in 0..q {
        input! {
            k: usize,
            b: [Usize1; k],
        }

        for &i in &indices {
            if !b.contains(&i) {
                ans.push(arr[i]);
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
