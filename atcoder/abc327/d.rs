#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let us = readv::<usize>();
    let vs = readv::<usize>();
    let mut adj = vec![vec![]; n];
    for i in 0..m {
        let u = us[i] - 1;
        let v = vs[i] - 1;
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut color = vec![2; n];
    let mut que = std::collections::VecDeque::new();
    for root in 0..n {
        if color[root] == 2 {
            color[root] = 0;
            que.push_back(root);
            while let Some(u) = que.pop_front() {
                for &v in adj[u].iter() {
                    if color[v] == 2 {
                        color[v] = 1 - color[u];
                        que.push_back(v);
                    }
                }
            }
        }
    }

    let mut ok = true;
    for u in 0..n {
        for &v in adj[u].iter() {
            ok &= (color[u] != color[v]);
        }
    }

    if ok {
        println!("Yes");
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
