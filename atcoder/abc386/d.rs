#![allow(unused)]

use std::collections::BTreeSet;

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut cells = vec![];
    for _ in 0..m {
        let inp = readv::<String>();
        let r = inp[0].parse::<usize>().unwrap() - 1;
        let c = inp[1].parse::<usize>().unwrap() - 1;
        let k = inp[2].chars().next().unwrap();
        cells.push((r, c, k));
    }

    // Are there any white at the top-left of some black?
    // => Sweep line
    cells.sort();

    let mut set = BTreeSet::new();
    for (r, c, k) in cells {
        if k == 'W' {
            set.insert(c);
        } else {
            if let Some(x) = set.range(0..=c).last() {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
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
