#![allow(unused)]

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        arr_a: [Usize1; n],
        arr_b: [Usize1; n],
    }

    let mut owner = vec![None; n];
    for i in 0..n {
        owner[i] = Some(arr_b[i]);
    }

    if (0..n).all(|i| owner[arr_a[i]] == Some(i)) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
