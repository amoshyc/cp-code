#![allow(unused)]

use std::collections::BTreeSet;

fn main() {
    let mut set = BTreeSet::new();
    let mut add = 0;

    let q = read::<usize>();
    let mut ans = vec![];
    for qid in 0..q {
        let ask = readv::<i64>();
        if ask[0] == 1 {
            set.insert((-add, qid));
        } else if ask[0] == 2 {
            let t = ask[1];
            add += t;
        } else {
            let h = ask[1];
            let popped: Vec<_> = set.range((h - add, 0)..).cloned().collect();
            ans.push(popped.len());
            for (x, qid) in popped {
                set.remove(&(x, qid));
            }
        }
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
