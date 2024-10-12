#![allow(unused)]

use std::collections::HashSet;

fn main() {
    let n = read::<usize>();

    let mut e1 = HashSet::new();
    for _ in 0..read::<usize>() {
        let uv = readv::<usize>();
        e1.insert((uv[0] - 1, uv[1] - 1));
    }

    let mut e2 = HashSet::new();
    for _ in 0..read::<usize>() {
        let uv = readv::<usize>();
        e2.insert((uv[0] - 1, uv[1] - 1));
    }

    let mut arr = vec![vec![10i64.pow(14); n]; n];
    for r in 0..(n - 1) {
        let row = readv::<i64>();
        for i in 0..row.len() {
            arr[r][r + 1 + i] = row[i];
            arr[r + 1 + i][r] = row[i];
        }
    }

    let norm = |a: usize, b: usize| (a.min(b), a.max(b));
    let mut perm: Vec<usize> = (0..n).collect();
    let mut ans = i64::MAX;
    loop {
        let mut edges: HashSet<_> = e1.iter().map(|&(u, v)| norm(perm[u], perm[v])).collect();

        let mut cost = 0;
        for &(u, v) in e2.iter() {
            if !edges.contains(&norm(u, v)) {
                cost += arr[u][v];
            }
        }
        for &(u, v) in edges.iter() {
            if !e2.contains(&norm(u, v)) {
                cost += arr[u][v];
            }
        }

        ans = ans.min(cost);

        if next_permutation(&mut perm).is_none() {
            break;
        }
    }

    println!("{}", ans);
}

fn next_permutation<T: Ord>(arr: &mut [T]) -> Option<()> {
    let k = arr.windows(2).rposition(|w| w[0] < w[1])?;
    let j = arr.iter().rposition(|a| a > &arr[k]).unwrap();
    arr.swap(k, j);
    arr[(k + 1)..].reverse();
    Some(())
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
