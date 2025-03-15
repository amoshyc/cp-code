#![allow(unused)]

use std::collections::VecDeque;

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut adj = vec![vec![]; n];
    for _ in 0..m {
        let xyz = readv::<u64>();
        let x = xyz[0] as usize - 1;
        let y = xyz[1] as usize - 1;
        let z = xyz[2];
        adj[x].push((y, z));
        adj[y].push((x, z));
    }

    // For each cc, given a value x for some vertex, we can determine the value of all other.
    // To minimize the total value, x has to be determined optimally.
    // Note that in XOR, each bit is independent of each other.

    // To find the i-th bit of x,
    // we can check the number of inversion of i-th bit for all vertices in the cc.

    let mut val = vec![!0; n];
    for r in 0..n {
        if val[r] != !0 {
            continue;
        }

        // Find the cc and the value of vertices assuming x = 0
        val[r] = 0;
        let mut que = VecDeque::from([r]);
        let mut cc = vec![];
        while let Some(u) = que.pop_front() {
            cc.push(u);
            for &(v, z) in &adj[u] {
                if val[v] == !0 {
                    val[v] = val[u] ^ z;
                    que.push_back(v);
                }
            }
        }

        // Determine x
        let mut x = 0;
        for i in 0..32 {
            let count_same = cc.iter().filter(|&&v| (val[v] >> i) & 1 == 0).count();
            let count_invr = cc.len() - count_same;
            if count_same < count_invr {
                x |= 1 << i;
            }
        }

        // Apply the x to the cc
        for &u in &cc {
            val[u] ^= x;
        }
    }

    if (0..n).all(|u| adj[u].iter().all(|&(v, z)| val[u] ^ val[v] == z)) {
        println!("{}", join(&val, " "));
    } else {
        println!("-1");
    }
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
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
