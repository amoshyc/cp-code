#![allow(unused)]

fn solve() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut adj = vec![vec![]; n];
    for _ in 0..m {
        let inp = readv::<usize>();
        let (u, v) = (inp[0] - 1, inp[1] - 1);
        adj[u].push(v);
        adj[v].push(u);
    }

    let (nodes, _, _) = bfs(&adj, vec![0]);
    let s = nodes[nodes.len() - 1];
    let (nodes, _, dep1) = bfs(&adj, vec![s]);
    let t = nodes[nodes.len() - 1];
    let (nodes, _, dep2) = bfs(&adj, vec![t]);
    let ecc = (0..n).map(|u| dep1[u].max(dep2[u])).collect::<Vec<_>>();

    let center = (0..n).min_by_key(|&i| ecc[i]).unwrap();
    let x = adj[center].len();
    let y = (n - x - 1) / x;
    println!("{} {}", x, y);
}

fn main() {
    let tc = read::<usize>();
    for _ in 0..tc {
        solve();
    }
}

fn bfs(adj: &Vec<Vec<usize>>, mut roots: Vec<usize>) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
    let n = adj.len();
    let inf = std::usize::MAX;
    let mut nodes = vec![];
    let mut parent = vec![inf; n];
    let mut depth = vec![inf; n];
    let mut queue = std::collections::VecDeque::new();

    if roots.len() == 0 {
        roots.extend(0..n);
    }

    for root in roots {
        if parent[root] == inf {
            parent[root] = root;
            depth[root] = 0;
            queue.push_back(root);
            while let Some(u) = queue.pop_front() {
                nodes.push(u);
                for &v in adj[u].iter() {
                    if parent[v] == inf {
                        parent[v] = u;
                        depth[v] = depth[u] + 1;
                        queue.push_back(v);
                    }
                }
            }
        }
    }

    (nodes, parent, depth)
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
