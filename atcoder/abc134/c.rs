#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        arr: [i64; n]
    }

    let mut indices = (0..n).collect::<Vec<usize>>();
    indices.sort_by_key(|i| arr[*i]);

    let mut ans = vec![arr[indices[n - 1]]; n];
    ans[indices[n - 1]] = arr[indices[n - 2]];
    println!("{}", join(&ans, "\n"));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
