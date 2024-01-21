#![allow(unused)]

use std::collections::{BTreeSet, HashMap};

fn main() {
    let n = read::<usize>();
    let q = read::<usize>();
    let mut boxes = HashMap::new();
    let mut belong = HashMap::new();

    for qid in 0..q {
        let inp = readv::<usize>();

        if inp[0] == 1 {
            let (i, j) = (inp[1], inp[2]);
            boxes.entry(j).or_insert(BTreeSet::new()).insert((i, qid));
            belong.entry(i).or_insert(BTreeSet::new()).insert(j);
        } else if inp[0] == 2 {
            let ans = boxes[&inp[1]].iter().map(|&(x, _)| x).collect::<Vec<_>>();
            println!("{}", join(&ans, " "));
        } else {
            let ans = belong[&inp[1]].iter().collect::<Vec<_>>();
            println!("{}", join(&ans, " "));
        }
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
    read::<String>().chars().collect::<Vec<char>>()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
