#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, q) = (inp[0], inp[1]);

    let mut adj = vec![vec![]; n];
    for _ in 0..(n - 1) {
        let inp = readv::<usize>();
        let (u, v) = (inp[0] - 1, inp[1] - 1);
        adj[u].push(v);
        adj[v].push(u);
    }

    let lca = LCA::from_adj(&adj);

    let mut ans = vec![];
    for _ in 0..q {
        let ask = readv::<usize>();
        let (u, v) = (ask[0] - 1, ask[1] - 1);
        let d = lca.dist(u, v);
        if d % 2 == 0 {
            ans.push("Town");
        } else {
            ans.push("Road");
        }
    }

    println!("{}", join(&ans, "\n"));
}

struct LCA {
    root: usize,
    dep: Vec<usize>,
    dp: Vec<Vec<usize>>,
}

impl LCA {
    fn from_adj(adj: &Vec<Vec<usize>>) -> Self {
        let n = adj.len();
        let inf = usize::MAX;
        let mut que = std::collections::VecDeque::new();
        let mut dep = vec![inf; n];
        let mut par = (0..n).collect::<Vec<usize>>();
        let root = 0;
        que.push_back(root);
        dep[root] = 0;
        par[root] = root;
        while let Some(u) = que.pop_front() {
            for &v in adj[u].iter() {
                if dep[v] == inf {
                    dep[v] = dep[u] + 1;
                    par[v] = u;
                    que.push_back(v);
                }
            }
        }
        assert!(dep.iter().all(|d| *d != inf));

        let nn = n.next_power_of_two().trailing_zeros() as usize;
        let mut dp = vec![vec![root; n]; nn]; // beyond root is root
        dp[0].clone_from_slice(&par);
        for i in 1..nn {
            for u in 0..n {
                dp[i][u] = dp[i - 1][dp[i - 1][u]];
            }
        }
        Self { root, dep, dp }
    }

    fn kth_par(&self, mut u: usize, k: usize) -> usize {
        for i in 0..self.dp.len() {
            if (k >> i) & 1 == 1 {
                u = self.dp[i][u];
            }
        }
        u
    }

    fn lca(&self, mut u: usize, mut v: usize) -> usize {
        // Make u lower that v
        if self.dep[u] < self.dep[v] {
            (u, v) = (v, u);
        }
        // Make u, v same depth
        u = self.kth_par(u, self.dep[u] - self.dep[v]);
        if u == v {
            return u;
        }
        // Increment binary search
        for i in (0..self.dp.len()).rev() {
            if self.dp[i][u] != self.dp[i][v] {
                u = self.dp[i][u];
                v = self.dp[i][v];
            }
        }
        self.dp[0][u]
    }

    fn dist(&self, u: usize, v: usize) -> usize {
        let lca = self.lca(u, v);
        self.dep[u] + self.dep[v] - 2 * self.dep[lca]
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
