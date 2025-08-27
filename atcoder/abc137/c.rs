#![allow(unused)]

use std::collections::HashMap;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        mut arr: [Chars; n],
    }

    for i in 0..n {
        arr[i].sort();
    }

    let mut ans = 0;
    let mut cnt = HashMap::new();
    for i in 0..n {
        ans += *cnt.get(&arr[i]).unwrap_or(&0) as i64;
        *cnt.entry(&arr[i]).or_insert(0) += 1;
    }

    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
