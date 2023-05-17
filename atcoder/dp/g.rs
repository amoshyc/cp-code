#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut adj = vec![vec![]; n];
    for _ in 0..m {
        let inp = readv::<usize>();
        let (u, v) = (inp[0] - 1, inp[1] - 1);
        adj[u].push(v);
    }

    // dp[u] = lenght of the longest path ending at u
    // dp[u] = max(dp[v] + 1 for v in child(u))
    let mut dp = vec![0; n];

    let mut sort = topological_sort(&adj);
    sort.reverse();
    dp[sort[0]] = 0;
    for &u in sort[1..].iter() {
        for &v in adj[u].iter() {
            dp[u] = dp[u].max(dp[v] + 1);
        }
    }

    println!("{}", dp.iter().max().unwrap());
}

fn topological_sort(adj: &Vec<Vec<usize>>) -> Vec<usize> {
    let n = adj.len();
    let mut indeg = vec![0; n];
    for u in 0..n {
        for &v in adj[u].iter() {
            indeg[v] += 1;
        }
    }

    let mut que = std::collections::VecDeque::new();
    for u in 0..n {
        if indeg[u] == 0 {
            que.push_back(u);
        }
    }

    let mut nodes = vec![];
    while let Some(u) = que.pop_front() {
        nodes.push(u);
        for &v in adj[u].iter() {
            indeg[v] -= 1;
            if indeg[v] == 0 {
                que.push_back(v);
            }
        }
    }

    nodes
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
