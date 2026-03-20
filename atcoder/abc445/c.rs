#![allow(unused)]

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        mut nxt: [Usize1; n],
    }

    for i in (0..n).rev() {
        if nxt[i] == i {
            continue;
        } else {
            nxt[i] = nxt[nxt[i]];
        }
    }

    let ans = nxt.iter().map(|&x| x + 1).collect::<Vec<_>>();
    println!("{}", join(&ans, " "));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
