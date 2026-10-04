#![allow(unused)]

use std::collections::BinaryHeap;

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        n: usize,
        arr: [usize; n],
    }

    let mut ans = vec![];
    let mut que = BinaryHeap::new();
    que.push(arr[0]);
    que.push(arr[1]);
    
    for i in 2..n {
        que.push(arr[i]);

        let a = que.pop().unwrap();
        let b = que.pop().unwrap();
        let c = que.pop().unwrap();
        ans.push(c);

        que.push(c);
        que.push(b);
        que.push(a);
    }

    println!("{}", join(&ans, "\n"));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
