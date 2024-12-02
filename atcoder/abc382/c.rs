#![allow(unused)]

use std::collections::BTreeSet;

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let arr_a = readv::<usize>();
    let arr_b = readv::<usize>();

    let mut inv_a = vec![vec![]; 200_000 + 1];
    for i in 0..n {
        inv_a[arr_a[i]].push(i);
    }

    let mut indices: Vec<usize> = (0..m).collect();
    indices.sort_by_key(|i| arr_b[*i]);

    let mut set = BTreeSet::new();
    let mut ans = vec![-1; m];
    let mut lvl = 0;
    for i in indices {
        let val_b = arr_b[i];
        while lvl <= val_b {
            for j in inv_a[lvl].iter() {
                set.insert(j);
            }
            lvl += 1;
        }

        if let Some(j) = set.first().cloned() {
            ans[i] = (j + 1) as i32;
            // set.remove(&j); // if one-to-one matching
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
