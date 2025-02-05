#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut adj = vec![vec![]; n];
    for _ in 0..m {
        let uv = readv::<usize>();
        let (u, v) = (uv[0] - 1, uv[1] - 1);
        adj[u].push(v);
    }

    let inf = usize::MAX;
    let (dep, _) = bfs(&adj, vec![0], inf);

    let mut ans = inf;
    for u in 1..n {
        if dep[u] != inf {
            if adj[u].contains(&0) {
                ans = ans.min(dep[u] + 1);
            }
        }
    }

    if ans == inf {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}

fn bfs(adj: &Vec<Vec<usize>>, srcs: Vec<usize>, inf: usize) -> (Vec<usize>, Vec<usize>) {
    let n = adj.len();
    let mut dep = vec![inf; n];
    let mut par = vec![!0; n];
    let mut que = std::collections::VecDeque::new();

    for src in srcs {
        dep[src] = 0;
        par[src] = src;
        que.push_back(src);
    }

    while let Some(u) = que.pop_front() {
        for &v in adj[u].iter() {
            if dep[v] == inf {
                dep[v] = dep[u] + 1;
                par[v] = u;
                que.push_back(v);
            }
        }
    }

    (dep, par)
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
