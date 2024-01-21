#![allow(unused)]

use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    let inp = readv::<usize>();
    let (h, w, n) = (inp[0], inp[1], inp[2]);
    let inp = readv::<usize>();
    let (sr, sc) = (inp[0] - 1, inp[1] - 1);
    let inp = readv::<usize>();
    let (gr, gc) = (inp[0] - 1, inp[1] - 1);

    let mut rows = HashMap::new();
    let mut cols = HashMap::new();
    for _ in 0..n {
        let inp = readv::<usize>();
        let (r, c) = (inp[0] - 1, inp[1] - 1);
        rows.entry(r).or_insert(BTreeSet::new()).insert(c);
        cols.entry(c).or_insert(BTreeSet::new()).insert(r);
    }

    let neighbors = |r: usize, c: usize| {
        let mut res = vec![];
        if let Some(s) = rows.get(&r) {
            // right
            if let Some(x) = s.range(c..).next() {
                res.push((r, *x - 1));
            }
            // left
            if let Some(x) = s.range(..c).last() {
                res.push((r, *x + 1));
            }
        }
        if let Some(s) = cols.get(&c) {
            // down
            if let Some(x) = s.range(r..).next() {
                res.push((*x - 1, c));
            }
            // top
            if let Some(x) = s.range(..r).last() {
                res.push((*x + 1, c));
            }
        }
        res
    };

    let inf = 10i64.pow(18);
    let mut dis = HashMap::new();
    let mut que = VecDeque::new();
    dis.insert((sr, sc), 0);
    que.push_back((sr, sc));
    while let Some((r, c)) = que.pop_front() {
        for (nr, nc) in neighbors(r, c) {
            if dis[&(r, c)] + 1 < *dis.get(&(nr, nc)).unwrap_or(&inf) {
                dis.insert((nr, nc), dis[&(r, c)] + 1);
                que.push_back((nr, nc));
            }
        }
    }

    if dis.contains_key(&(gr, gc)) {
        println!("{}", dis[&(gr, gc)]);
    } else {
        println!("-1");
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
