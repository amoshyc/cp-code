#![allow(unused)]

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        q: usize,
    }

    let mut ans = vec![];
    let mut heap = BinaryHeap::new();
    for _ in 0..q {
        input! {
            cmd: usize
        }

        if cmd == 1 {
            input! { x: usize }
            heap.push(Reverse(x));
        } else {
            let Reverse(x) = heap.pop().unwrap();
            ans.push(x);
        }
    }

    println!("{}", join(&ans, "\n"));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
