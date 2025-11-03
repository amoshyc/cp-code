#![allow(unused)]

use std::collections::HashMap;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut cnt = HashMap::new();
    for c in s {
        *cnt.entry(c).or_insert(0) += 1;
    }

    for (k, v) in cnt {
        if v == 1 {
            println!("{}", k);
            return;
        }
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
