#![allow(unused)]

use std::collections::HashSet;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        mut arr: [i64; n],
    }

    let mut vis = HashSet::new();
    let mut pos = vec![];
    for i in 0..n {
        if arr[i] != -1 {
            vis.insert(arr[i]);
        } else {
            pos.push(i);
        }
    }

    if n - vis.len() == pos.len() {
        println!("Yes");
        for i in 1..=n {
            if !vis.contains(&(i as i64)) {
                arr[pos.pop().unwrap()] = i as i64;
            }
        }
        println!("{}", join(&arr, " "));
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
