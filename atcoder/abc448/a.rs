#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut x: usize,
        mut arr: [usize; n],
    }

    for i in 0..n {
        if arr[i] < x {
            x = arr[i];
            arr[i] = 1;
        } else {
            arr[i] = 0;
        }
    }

    println!("{}", join(&arr, "\n"));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
