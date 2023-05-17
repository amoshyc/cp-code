#![allow(unused)]

use std::collections::HashSet;

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    
    let mut adj = vec![vec![]; n];
    for i in 0..m {
        let inp = readv::<usize>();
        let (u, v) = (inp[0] - 1, inp[1] - 1);
        adj[u].push((v, i));
        adj[v].push((u, i));
    }

    // println!("{:?}", adj);

    let mut vis = vec![false; n];
    for root in 0..n {
        if vis[root] {
            continue;
        }

        let mut stack = vec![root];
        let mut vert_set = HashSet::new();
        let mut edge_set = HashSet::new();
        while let Some(u) = stack.pop() {
            vert_set.insert(u);
            for &(v, eid) in adj[u].iter() {
                edge_set.insert(eid);
                if !vis[v] {
                    vis[v] = true;
                    stack.push(v);
                }
            }
        }

        // println!("{}", root);
        // println!("vert: {:?}", vert_set);
        // println!("edge: {:?}", edge_set);

        if vert_set.len() != edge_set.len() {
            println!("No");
            return;
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
    read::<String>().chars().collect::<Vec<char>>()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
