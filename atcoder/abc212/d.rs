#![allow(unused)]

use std::collections::BTreeSet;

fn main() {
    let q = read::<usize>();
    let mut set = BTreeSet::new();
    let mut inc: i64 = 0;
    let mut ans = vec![];
    for qid in 0..q {
        let inp = readv::<i64>();
        if inp[0] == 1 {
            set.insert((inp[1] - inc, qid));
        } else if inp[0] == 2 {
            inc += inp[1];
        } else {
            let (x, i) = *set.iter().next().unwrap();
            set.remove(&(x, i));
            ans.push(x + inc);
        }
    }
    println!("{}", join(&ans, "\n"));
}


fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read2<T: std::str::FromStr, F: std::str::FromStr>() -> (T, F) {
    let mut inp = read::<String>();
    let mut token = inp.split_ascii_whitespace();
    let a: T = token.next().unwrap().parse().ok().unwrap();
    let b: F = token.next().unwrap().parse().ok().unwrap();
    (a, b)
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
