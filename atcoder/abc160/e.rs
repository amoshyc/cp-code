#![allow(unused)]

use std::cmp::Reverse;

fn main() {
    let inp = readv::<usize>();
    let (x, y) = (inp[0], inp[1]);
    let (a, b, c) = (inp[2], inp[3], inp[4]);
    let p = readv::<i64>();
    let q = readv::<i64>();
    let r = readv::<i64>();

    let mut red = std::collections::BinaryHeap::new();
    let mut grn = std::collections::BinaryHeap::new();
    for i in 0..a {
        red.push(Reverse(p[i]));
        if red.len() > x {
            red.pop();
        }
    }
    for i in 0..b {
        grn.push(Reverse(q[i]));
        if grn.len() > y {
            grn.pop();
        }
    }

    for i in 0..c {
        let Reverse(u) = red.peek().unwrap();
        let Reverse(v) = grn.peek().unwrap();
        if u < v {
            red.push(Reverse(r[i]));
            red.pop();
        } else {
            grn.push(Reverse(r[i]));
            grn.pop();
        }
    }

    let mut ans = 0;
    for &Reverse(x) in red.iter() {
        ans += x;
    }
    for &Reverse(x) in grn.iter() {
        ans += x;
    }
    println!("{}", ans);
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
