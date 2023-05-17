#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut adj = vec![vec![]; n];
    let mut edges = vec![];
    for _ in 0..n - 1 {
        let inp = readv::<usize>();
        let (u, v, w) = (inp[0] - 1, inp[1] - 1, inp[2] as i64);
        adj[u].push(v);
        adj[v].push(u);
        edges.push((u, v, w));
    }

    let hld = HLD::new(&adj, 0);

    let mut bit = Bit::<i64>::new(n + 1);
    for &(u, v, w) in edges.iter() {
        let (u, v) = if hld.parent[u] == v { (v, u) } else { (u, v) };
        bit.add(hld.t_enter[v], w);
        bit.add(hld.t_leave[v], -w);
    }

    let q = read::<usize>();
    let mut ans = vec![];
    for _ in 0..q {
        let inp = readv::<usize>();
        let (c, a, b) = (inp[0], inp[1], inp[2]);

        if c == 1 {
            let (u, v, w) = edges[a - 1];
            let (u, v) = if hld.parent[u] == v { (v, u) } else { (u, v) };
            let b = b as i64;
            bit.add(hld.t_enter[v], b - w);
            bit.add(hld.t_leave[v], -b + w);
            edges[a - 1] = (u, v, b);
        } else {
            let (a, b) = (a - 1, b - 1);
            let lca = hld.lca(a, b);
            let da = bit.sum(0, hld.t_enter[a] + 1);
            let db = bit.sum(0, hld.t_enter[b] + 1);
            let dl = bit.sum(0, hld.t_enter[lca] + 1);
            let v = da + db - 2 * dl;
            ans.push(v);
        }
    }

    println!("{}", join(&ans, "\n"));
}

#[derive(Debug)]
struct HLD {
    size: Vec<usize>,
    depth: Vec<usize>,
    heavy: Vec<usize>,
    parent: Vec<usize>,

    top: Vec<usize>,
    tour: Vec<usize>,
    t_enter: Vec<usize>,
    t_leave: Vec<usize>,
}

impl HLD {
    fn new(adj: &Vec<Vec<usize>>, root: usize) -> Self {
        let n = adj.len();

        // DFS to find size, depth, heavy, parent
        let mut size = vec![1; n];
        let mut depth = vec![0; n];
        let mut heavy = vec![std::usize::MAX; n];
        let mut parent = vec![std::usize::MAX; n];
        let mut stack = vec![('l', root), ('e', root)];
        depth[root] = 0;
        parent[root] = root;
        while let Some((cmd, u)) = stack.pop() {
            if cmd == 'l' {
                size[u] = 1;
                heavy[u] = std::usize::MAX;
                for &v in adj[u].iter().rev() {
                    if v != parent[u] {
                        size[u] += size[v];
                        if heavy[u] == std::usize::MAX || size[v] > size[heavy[u]] {
                            heavy[u] = v;
                        }
                    }
                }
            } else {
                for &v in adj[u].iter().rev() {
                    if parent[v] == std::usize::MAX {
                        parent[v] = u;
                        depth[v] = depth[u] + 1;
                        stack.push(('l', v));
                        stack.push(('e', v));
                    }
                }
            }
        }

        // DFS to decompose tree
        let mut time = 0;
        let mut top = vec![std::usize::MAX; n];
        let mut tour = vec![std::usize::MAX; n];
        let mut t_enter = vec![std::usize::MAX; n];
        let mut t_leave = vec![std::usize::MAX; n];
        let mut stack = vec![('l', root), ('e', root)];
        top[root] = root;
        while let Some((cmd, u)) = stack.pop() {
            if cmd == 'l' {
                t_leave[u] = time;
            } else {
                t_enter[u] = time;
                tour[time] = u;
                time += 1;
                for &v in adj[u].iter().rev() {
                    if v != parent[u] {
                        top[v] = if v == heavy[u] { top[u] } else { v };
                        stack.push(('l', v));
                        stack.push(('e', v));
                    }
                }
            }
        }

        Self {
            size,
            depth,
            heavy,
            parent,
            top,
            tour,
            t_enter,
            t_leave,
        }
    }

    fn lca(&self, mut u: usize, mut v: usize) -> usize {
        assert!(u < self.top.len());
        assert!(v < self.top.len());
        // Until they are in the same path
        while self.top[u] != self.top[v] {
            if self.depth[self.top[u]] > self.depth[self.top[v]] {
                u = self.parent[self.top[u]];
            } else {
                v = self.parent[self.top[v]];
            }
        }
        if self.depth[u] < self.depth[v] {
            u
        } else {
            v
        }
    }
}

struct Bit<T> {
    data: Vec<T>,
}

impl<T> Bit<T>
where
    T: Copy + Default + std::ops::Add<Output = T> + std::ops::Sub<Output = T>,
{
    fn new(n: usize) -> Self {
        Self {
            data: vec![T::default(); n + 1],
        }
    }

    fn add(&mut self, idx: usize, val: T) {
        let n = self.data.len() - 1;
        assert!(idx < n);
        let mut i = idx + 1;
        while i <= n {
            self.data[i] = self.data[i] + val;
            i += i & (!i + 1);
        }
    }

    fn prefix(&self, idx: usize) -> T {
        let n = self.data.len() - 1;
        assert!(idx < n);
        let mut res = T::default();
        let mut i = idx + 1;
        while i > 0 {
            res = res + self.data[i];
            i -= i & (!i + 1);
        }
        res
    }

    // l..r
    fn sum(&self, l: usize, r: usize) -> T {
        let n = self.data.len() - 1;
        assert!(l < r && r <= n);
        let val = self.prefix(r - 1);
        if l == 0 {
            val
        } else {
            val - self.prefix(l - 1)
        }
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
