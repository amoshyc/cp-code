#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);

    let mut ans = 100;
    let mut deg = vec![0; n];
    let mut given = 0u64;
    for _ in 0..m {
        let uv = readv::<usize>();
        let (u, v) = (uv[0] - 1, uv[1] - 1);
        if u < v {
            given |= 1 << (u * n + v);
        } else {
            given |= 1 << (v * n + u);
        }
    }
    dfs(0, 1, &mut deg, 0, given, &mut ans);

    println!("{}", ans);
}

// Consider if we want edge (u, v)
fn dfs(u: usize, v: usize, deg: &mut Vec<usize>, edges: u64, given: u64, ans: &mut usize) {
    // Arrive at target state
    if deg.iter().all(|x| *x == 2) {
        let cnt = (edges ^ given).count_ones() as usize;
        *ans = (*ans).min(cnt);
        return;
    }

    // All edges examined
    let n = deg.len();
    if u == n || v == n {
        return;
    }

    // u < v
    let (mut next_u, mut next_v) = (u, v);
    next_v += 1;
    if next_v == n {
        next_u += 1;
        next_v = next_u + 1;
    }

    // edge (u, v) is used
    if deg[u] < 2 && deg[v] < 2 && kth_bit(edges, u * n + v) == 0 {
        deg[u] += 1;
        deg[v] += 1;
        dfs(next_u, next_v, deg, edges | (1 << (u * n + v)), given, ans);
        deg[u] -= 1;
        deg[v] -= 1;
    }

    // don't use edge (u, v)
    dfs(next_u, next_v, deg, edges, given, ans);
}

fn kth_bit<T>(x: T, k: usize) -> T
where
    T: std::ops::BitAnd<Output=T> + std::ops::Shr<usize, Output=T> + From<u8>,
{
    (x >> k) & T::from(1)
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
