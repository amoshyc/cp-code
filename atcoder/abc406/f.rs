#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut adj = vec![vec![]; n];
    let mut edges = vec![(0, 0); n - 1];
    for eid in 0..(n - 1) {
        let uv = readv::<usize>();
        let (u, v) = (uv[0] - 1, uv[1] - 1);
        adj[u].push(v);
        adj[v].push(u);
        edges[eid] = (u, v);
    }

    let (enter, leave) = euler_tour(&adj, 0);

    let mut bit = BIT::<i64>::new(n);
    for u in 0..n {
        bit.add(enter[u], 1);
    }

    let q = read::<usize>();
    let mut ans = vec![];
    for _ in 0..q {
        let ask = readv::<i64>();
        if ask[0] == 1 {
            let (x, w) = (ask[1] as usize - 1, ask[2]);
            bit.add(enter[x], w);
        } else {
            let eid = ask[1] as usize - 1;
            let (mut u, mut v) = edges[eid];
            if enter[u] > enter[v] {
                (u, v) = (v, u);
            }
            let sum1 = bit.sum(enter[v], leave[v]);
            let sum2 = bit.sum(0, n) - sum1;
            ans.push(sum1.abs_diff(sum2));
        }
    }

    println!("{}", join(&ans, "\n"));
}

// subtree of u <-> range enter[u]..leave[u]
fn euler_tour(adj: &Vec<Vec<usize>>, root: usize) -> (Vec<usize>, Vec<usize>) {
    let n = adj.len();
    let mut t = 0;
    let mut enter = vec![!0; n]; // enter time
    let mut leave = vec![!0; n]; // leave time
    euler_dfs(root, !0, &mut t, &mut enter, &mut leave, adj);
    (enter, leave)
}

fn euler_dfs(
    u: usize,
    p: usize,
    t: &mut usize,
    enter: &mut Vec<usize>,
    leave: &mut Vec<usize>,
    adj: &Vec<Vec<usize>>,
) {
    enter[u] = *t;
    *t += 1;
    for &v in &adj[u] {
        if v != p {
            euler_dfs(v, u, t, enter, leave, adj);
        }
    }
    leave[u] = *t;
}

struct BIT<T> {
    dat: Vec<T>,
}

impl<T: Clone + Default + std::ops::AddAssign + std::ops::Sub<Output = T>> BIT<T> {
    fn new(n: usize) -> Self {
        Self {
            dat: vec![T::default(); n + 1],
        }
    }

    // 0-based
    fn add(&mut self, mut i: usize, x: T) {
        i += 1; // convert to 1-based
        while i < self.dat.len() {
            self.dat[i] += x.clone();
            i += i & (!i + 1); // i & (-i)
        }
    }

    // 0..=i, 0-based
    fn pref(&self, mut i: usize) -> T {
        let mut res = T::default();
        i += 1; // convert to 1-based
        while i > 0 {
            res += self.dat[i].clone();
            i -= i & (!i + 1);
        }
        res
    }

    // l..i, 0-based
    fn sum(&self, mut l: usize, mut r: usize) -> T {
        if r == 0 {
            T::default()
        } else if l >= 1 {
            self.pref(r - 1) - self.pref(l - 1)
        } else {
            self.pref(r - 1)
        }
    }
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
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
