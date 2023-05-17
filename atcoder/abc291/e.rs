#![allow(unused)]

use std::iter::FromIterator;

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);

    let mut adj = vec![vec![]; n];
    let mut deg = vec![0; n];
    for _ in 0..m {
        let inp = readv::<usize>();
        let (u, v) = (inp[0] - 1, inp[1] - 1);
        adj[u].push(v);
        deg[v] += 1;
    }

    let mut roots = Vec::from_iter((0..n).filter(|&v| deg[v] == 0));
    let mut order = vec![0; n];
    let mut idx = 0;

    while roots.len() > 0 {
        if roots.len() != 1 {
            break;
        }

        let u = roots[0];
        order[u] = idx;
        idx += 1;

        let mut next = vec![];
        for &v in adj[u].iter() {
            deg[v] -= 1;
            if deg[v] == 0 {
                next.push(v);
            }
        }

        roots = next;
    }

    if idx == n {
        let order = order.iter().map(|&x| x + 1).collect::<Vec<_>>();
        println!("Yes");
        println!("{}", join(&order, " "));
    } else {
        println!("No");
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
