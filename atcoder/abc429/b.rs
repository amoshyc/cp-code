#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: i64,
        arr: [i64; n],
    }

    let sum = arr.iter().sum::<i64>();
    for i in 0..n {
        if sum - arr[i] == m {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
