#![allow(unused)]

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    }

    let mut deg = vec![0; n];
    for (u, v) in edges {
        deg[u] += 1;
        deg[v] += 1;
    }

    let mut ans = vec![0; n];
    for i in 0..n {
        let k = n as i64 - 1 - deg[i];
        if k >= 3 {
            ans[i] = k * (k - 1) * (k - 2) / 6;
        }
    }

    println!("{}", join(&ans, " "));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
