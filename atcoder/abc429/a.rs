#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut ans = vec![];
    for i in 0..n {
        if i < m {
            ans.push("OK");
        } else {
            ans.push("Too Many Requests");
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
