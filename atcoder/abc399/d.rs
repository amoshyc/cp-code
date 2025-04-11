#![allow(unused)]

use std::collections::{HashMap, HashSet};

fn main() {
    let tc = read::<usize>();

    let mut ans = vec![];
    for _ in 0..tc {
        let n = read::<usize>();
        let arr = readv::<usize>();

        let mut pos = HashMap::new();
        for i in 0..2 * n {
            pos.entry(arr[i]).or_insert(vec![]).push(i);
        }

        //    ...a...a...
        // => ..ba..ba...
        //    ..ba...ab..
        //    ...ab.ba...
        //    ...ab..ab..

        let mut cnt = 0;
        for (k, v) in pos {
            let (x, y) = (v[0], v[1]);
            if x + 1 == y {
                continue;
            }

            let mut vis = HashSet::new();
            for (dx, dy) in [(-1, -1), (-1, 1), (1, -1), (1, 1)] {
                let nx = x.checked_add_signed(dx).unwrap_or(2 * n);
                let ny = y.checked_add_signed(dy).unwrap_or(2 * n);
                if nx < 2 * n && ny < 2 * n {
                    if nx.min(ny) + 1 < nx.max(ny) && arr[nx] == arr[ny] && arr[nx] > k {
                        vis.insert(arr[nx]);
                    }
                }
            }
            cnt += vis.len();
        }
        ans.push(cnt);
    }

    println!("{}", join(&ans, "\n"));
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    s.trim().parse().ok().unwrap()
}

fn readv<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_ascii_whitespace()
        .map(|t| t.parse().ok().unwrap())
        .collect()
}

fn reads() -> Vec<char> {
    read::<String>().chars().collect()
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
