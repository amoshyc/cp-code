#![allow(unused)]

use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        arr: [usize; n],
    }

    let mut cnt = HashMap::new();
    for i in 0..n {
        *cnt.entry(arr[i]).or_insert(0) += 1;
    }

    if cnt.len() == n {
        println!("Yes");
    } else {
        println!("No");
    }

    if (1..=m).all(|x| *cnt.get(&x).unwrap_or(&0) >= 1) {
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
