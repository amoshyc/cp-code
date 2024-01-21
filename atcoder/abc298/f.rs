#![allow(unused)]

use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let n = read::<usize>();
    let mut points = vec![];
    let mut rows = vec![];
    let mut blacklists = HashMap::new();
    for _ in 0..n {
        let inp = readv::<u64>();
        let (r, c, x) = (inp[0], inp[1], inp[2]);
        rows.push(r);
        points.push((r, c, x));
        blacklists.entry(r).or_insert(HashSet::new()).insert(c);
    }

    let mut ans = 0;

    let mut col_sum = HashMap::new();
    let mut row_sum = HashMap::new();
    for &(r, c, x) in points.iter() {
        *row_sum.entry(r).or_insert(0) += x;
        *col_sum.entry(c).or_insert(0) += x;
    }
    for &(r, c, x) in points.iter() {
        ans = ans.max(row_sum[&r] + col_sum[&c] - x);
    }

    let mut cands = BTreeSet::new();
    for &(r, c, x) in points.iter() {
        cands.insert((col_sum[&c], c));
    }

    rows.sort();
    rows.dedup();

    for &r in rows.iter() {
        let mut popped = vec![];

        while let Some(&(s, c)) = cands.range(..).last() {
            if blacklists[&r].contains(&c) {
                popped.push((s, c));
                cands.remove(&(s, c));
            } else {
                ans = ans.max(row_sum[&r] + s);
                break;
            }
        }

        cands.extend(popped.iter());
    }

    println!("{}", ans);
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
