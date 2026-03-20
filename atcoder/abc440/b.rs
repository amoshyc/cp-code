#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        arr: [usize; n],
    }

    let mut ans: Vec<usize> = (1..=n).collect();
    ans.sort_by_key(|&i| arr[i - 1]);
    println!("{}", join(&ans[..3], " "));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
