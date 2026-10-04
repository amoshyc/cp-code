#![allow(unused)]

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        n: usize,
        s: Usize1,
        l: i64,
        dis: [i64; n - 1]
    }

    let mut pos = vec![0; n];
    for i in 0..(n - 1) {
        pos[i + 1] = pos[i] + dis[i];
    }

    // x <- s: pos[s] - pos[x]
    // x -> s -> y: pos[y] - pos[x]
    // total = pos[s] + pos[y] - 2 * pos[x] <= l
    // => Enumerate x, find y

    let mut ans = 0;
    for x in 0..=s {
        if pos[s] - pos[x] > l {
            continue;
        }
        ans = ans.max(s - x + 1);
        let yn = pos.partition_point(|&val| pos[s] + val - 2 * pos[x] <= l);
        if yn > s {
            ans = ans.max(yn - x);
        }
    }

    // s -> x: pos[x] - pos[s]
    // y <- s <- x: pos[x] - pos[y]
    // total = 2 * pos[x] - pos[s] - pos[y] <= l
    for x in s..n {
        if pos[x] - pos[s] > l {
            continue;
        }
        ans = ans.max(x - s + 1);
        let y = pos.partition_point(|&val| 2 * pos[x] - pos[s] - val > l);
        if y <= s {
            ans = ans.max(x - y + 1);
        }
    }

    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
