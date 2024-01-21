#![allow(unused)]

use std::collections::BTreeSet;

fn main() {
    let inp = readv::<usize>();
    let (n, q) = (inp[0], inp[1]);

    let mut s = BTreeSet::new();
    let mut w = 1;

    let mut ans = vec![];

    for _ in 0..q {
        let inp = readv::<usize>();
        let cmd = inp[0];
        let x = if inp.len() == 1 { n + 1 } else { inp[1] };

        match cmd {
            1 => {
                s.insert(w);
                w += 1;
            },
            2 => {
                s.remove(&x);
            },
            3 => {
                ans.push(s.iter().next().unwrap().clone());
            },
            _ => ()
        }
    }

    println!("{}", join(&ans, "\n"));
}

// arr.partition_point is added at 1.52.0
// 1 1 1 0 0 0
//       ^
fn partition_point<T, P: FnMut(&T) -> bool>(arr: &[T], mut pred: P) -> usize {
    arr.binary_search_by(|x| {
        if pred(x) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    })
    .unwrap_or_else(|i| i)
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
