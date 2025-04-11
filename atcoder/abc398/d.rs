#![allow(unused)]

use std::collections::HashSet;

fn main() {
    let inp = readv::<i64>();
    let (n, hr, hc) = (inp[0] as usize, inp[1], inp[2]);
    let s = reads();

    let mut ans = vec![0; n];
    let mut smokes = HashSet::new();

    let (mut or, mut oc) = (0, 0); // origin
    smokes.insert((or, oc));

    for i in 0..n {
        match s[i] {
            'N' => or += 1,
            'S' => or -= 1,
            'E' => oc -= 1,
            'W' => oc += 1,
            _ => (),
        }
        smokes.insert((or, oc));

        if smokes.contains(&(hr + or, hc + oc)) {
            ans[i] = 1;
        }
    }

    println!("{}", join(&ans, ""));
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
