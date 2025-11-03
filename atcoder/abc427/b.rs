#![allow(unused)]

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
    }

    let f = |mut x: u64| -> u64 {
        let mut cnt = 0;
        while x > 0 {
            cnt += x % 10;
            x /= 10;
        }
        cnt
    };

    let mut arr = vec![1; n + 1];
    for i in 1..=n {
        arr[i] = (0..i).map(|j| f(arr[j])).sum::<u64>();
    }

    println!("{}", arr[n]);
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
