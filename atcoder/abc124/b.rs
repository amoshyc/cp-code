#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        arr: [usize; n],
    }

    let ans = (0..n).filter(|&i| (0..i).all(|j| arr[j] <= arr[i])).count();
    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
