#![allow(unused)]

use std::collections::BTreeSet;

fn main() {
    let n = read::<usize>();
    let mut xys = vec![];
    for i in 0..n {
        let xy = readv::<i64>();
        xys.push((xy[0], xy[1], i + 1));
    }
    xys.sort();

    let mut set = BTreeSet::new();
    for (x, y, i) in xys {
        while set.len() > 0 {
            let (ty, tx, _) = set.iter().last().unwrap();
            if ty > &y {
                set.pop_last();
            } else {
                break;
            }
        }
        set.insert((y, x, i));
    }

    let mut ans = vec![];
    for (y, x, i) in set {
        ans.push(i);
    }
    ans.sort();

    println!("{}", ans.len());
    println!("{}", join(&ans, " "));
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
