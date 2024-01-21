#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut adj = vec![vec![]; n];
    let mut es = std::collections::HashMap::new();
    for i in 0..m {
        let inp = readv::<usize>();
        let (u, v) = (inp[0] - 1, inp[1] - 1);
        adj[u].push(v);
        es.insert((u, v), i);
    }

    let (dis, par) = bfs(&adj, 0, (!0, !0));
    if dis[n - 1] == !0 {
        let ans = vec![-1; m];
        println!("{}", join(&ans, "\n"));
        return;
    }

    let mut u = n - 1;
    let mut path = vec![];
    while u != 0 {
        path.push(u);
        u = par[u];
    }
    path.push(0);
    path.reverse();

    let mut ans = vec![dis[n - 1] as i32; m];
    for (i, w) in path.windows(2).enumerate() {
        let (u, v) = (w[0], w[1]);
        let (new_dis, _) = bfs(&adj, 0, (u, v));
        if new_dis[n - 1] == !0 {
            ans[es[&(u, v)]] = -1;
        } else {
            ans[es[&(u, v)]] = new_dis[n - 1] as i32;
        }
    }

    println!("{}", join(&ans, "\n"));
}

fn bfs(adj: &Vec<Vec<usize>>, root: usize, ban: (usize, usize)) -> (Vec<usize>, Vec<usize>) {
    let n = adj.len();
    let mut par: Vec<usize> = (0..n).collect();
    let mut dis: Vec<usize> = vec![!0; n];
    let mut que = std::collections::VecDeque::new();
    que.push_back(root);
    dis[root] = 0;
    par[root] = root;
    while let Some(u) = que.pop_front() {
        for &v in adj[u].iter() {
            if dis[u] + 1 < dis[v] && (u, v) != ban {
                dis[v] = dis[u] + 1;
                par[v] = u;
                que.push_back(v);
            }
        }
    }
    (dis, par)
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

fn mapv<T, S>(arr: &Vec<T>, f: fn(&T) -> S) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
