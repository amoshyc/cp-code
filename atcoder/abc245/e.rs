#![allow(unused)]

use std::cmp::Reverse;

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let a = readv::<u32>();
    let b = readv::<u32>();
    let c = readv::<u32>();
    let d = readv::<u32>();

    let mut xy = vec![];
    for i in 0..n {
        xy.push((a[i], b[i], i));
    }
    for i in 0..m {
        xy.push((c[i], d[i], n + i));
    }

    xy.sort_by_key(|&(x, y, t)| (Reverse(x), Reverse(y), Reverse(t)));
    let mut boxes = std::collections::BTreeSet::new();
    let mut ans = true;
    for (x, y, i) in xy {
        if i >= n {
            boxes.insert((y, i));
        } else {
            if let Some(&b) = boxes.range((y, 0)..).next() {
                boxes.remove(&b);
            } else {
                ans = false;
                break;
            }
        }
    }

    println!("{}", if ans { "Yes" } else { "No" });
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
