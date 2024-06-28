#![allow(unused)]

use std::collections::HashMap;

fn main() {
    let tc = read::<usize>();

    let mut ys = vec![];
    for y in (0..=100_001) {
        ys.push(y);
    }

    for _ in 0..tc {
        let r = read::<i64>();
        let mut cnt = 0;
        for x in -r..=r {
            // Find int y such that r <= sqrt(x^2 + y^2) < r + 1
            // Find int y such that r^2 <= x^2 + y^2 < (r + 1)^2
            // Find int y such that r^2 - x^2 <= y^2 < (r + 1)^2 - x^2
            let lb = ys[ys.partition_point(|&y| y * y < r * r - x * x)];
            let ub = ys[ys.partition_point(|&y| y * y < (r + 1) * (r + 1) - x * x)];
            let mut val = 2 * (ub - lb);
            if lb == 0 {
                val -= 1;
            }
            // println!("x={}, lb={}, ub={}, val={}", x, lb, ub, val);
            cnt += val;
        }
        println!("{}", cnt);
    }
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
    read::<String>().chars().collect::<_>()
}

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
