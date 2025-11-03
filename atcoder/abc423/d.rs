#![allow(unused)]

use std::collections::BTreeSet;
use std::collections::VecDeque;

use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        n: usize,
        k: i64,
        abc: [(i64, i64, i64); n],
    }

    let mut events = BTreeSet::new();
    for (i, &(a, b, c)) in abc.iter().enumerate() {
        events.insert((a, '+', b, c, i));
    }

    let mut ans = vec![0; n];
    let mut cnt = 0;
    let mut que = VecDeque::new();
    while let Some((t, f, b, c, i)) = events.pop_first() {
        if f == '+' {
            que.push_back((b, c, i));
        } else {
            cnt -= c;
        }
        while let Some(&(b, c, i)) = que.front() {
            if cnt + c <= k {
                cnt += c;
                ans[i] = t;
                events.insert((t + b, '-', b, c, i));
                que.pop_front();
            } else {
                break;
            }
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
