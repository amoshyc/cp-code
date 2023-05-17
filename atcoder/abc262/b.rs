#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);

    let mut adj = vec![vec![false; n]; n];
    for _ in 0..m {
        let inp = readv::<usize>();
        let (u, v) = (inp[0] - 1, inp[1] - 1);
        adj[u][v] = true;
        adj[v][u] = true;
    }

    let mut cnt = 0;
    for a in 0..n {
        for b in a + 1 .. n {
            for c in b + 1 ..n {
                if adj[a][b] && adj[b][c] && adj[a][c] {
                    cnt += 1;
                }
            }
        }
    }

    println!("{}", cnt);
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