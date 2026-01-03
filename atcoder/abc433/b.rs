#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        arr: [usize; n],
    }

    for i in 0..n {
        if let Some(j) = (0..i).rfind(|j| arr[*j] > arr[i]) {
            println!("{}", j + 1);
        } else {
            println!("-1");
        }
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
