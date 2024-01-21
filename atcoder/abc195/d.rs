#![allow(unused)]

use std::{cmp::Reverse, collections::BTreeSet};

fn main() {
    let inp = readv::<usize>();
    let (n, m, q) = (inp[0], inp[1], inp[2]);

    let mut items = vec![];
    for _ in 0..n {
        let inp = readv::<i64>();
        let (w, v) = (inp[0], inp[1]);
        items.push((w, v));
    }
    items.sort_by_key(|&(w, v)| (Reverse(v), w));

    let all_boxes = readv::<i64>();

    let mut ans = vec![];
    for _ in 0..q {
        let inp = readv::<usize>();
        let (l, r) = (inp[0] - 1, inp[1] - 1);

        let mut total = 0;
        let mut boxes = BTreeSet::new();
        for i in 0..m {
            if i < l || i > r {
                boxes.insert((all_boxes[i], i));
            }
        }
        for &(w, v) in items.iter() {
            if let Some(&(x, i)) = boxes.range((w, 0)..).next() {
                total += v;
                boxes.remove(&(x, i));
            }
        }

        ans.push(total);
    }

    println!("{}", join(&ans, "\n"));
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn readv<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_ascii_whitespace()
        .map(|t| t.parse().ok().unwrap())
        .collect()
}

fn reads() -> Vec<char> {
    read::<String>().chars().collect::<Vec<char>>()
}

fn mapv<T, S>(arr: &Vec<T>, f: fn(&T) -> S) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
