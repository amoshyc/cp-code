#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        t: i64,
        arr: [i64; n],
    }

    let mut close = 0;
    let mut i = 0;

    while i < n {
        let mut j = i;
        while j < n && arr[j] <= arr[i] + 100 {
            j += 1;
        }

        close += 100.min(t - arr[i]);

        i = j;
    }

    println!("{}", t - close);
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
