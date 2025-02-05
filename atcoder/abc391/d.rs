#![allow(unused)]

use std::cmp::Reverse;

fn main() {
    let inp = readv::<usize>();
    let (n, w) = (inp[0], inp[1]);

    let mut cols = vec![vec![]; w];
    for i in 0..n {
        let cr = readv::<usize>();
        let (c, r) = (cr[0] - 1, cr[1] - 1);
        cols[c].push((r, i));
    }

    let mut num_lines = n;
    for c in 0..w {
        cols[c].sort_by_key(|&(c, i)| (Reverse(c), i));
        num_lines = num_lines.min(cols[c].len());
    }

    let inf = usize::MAX;
    let mut eta = vec![inf; n];
    for _ in 0..num_lines {
        let mut time = 0;
        let mut line = vec![];
        for c in 0..w {
            let (r, i) = cols[c].pop().unwrap();
            line.push(i);
            time = time.max(r + 1);
        }
        for i in line {
            eta[i] = time;
        }
    }

    let q = read::<usize>();
    let mut ans = vec![];
    for _ in 0..q {
        let ask = readv::<usize>();
        let (t, x) = (ask[0], ask[1] - 1);
        if t < eta[x] {
            ans.push("Yes");
        } else {
            ans.push("No");
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
