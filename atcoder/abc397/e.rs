#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, k) = (inp[0], inp[1]);
    let mut adj = vec![vec![]; n * k];
    for _ in 0..(n * k - 1) {
        let uv = readv::<usize>();
        let (u, v) = (uv[0] - 1, uv[1] - 1);
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut ok = true;
    let mut size = vec![0; n * k];
    dfs(0, !0, &mut ok, &mut size, &adj, k);

    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn dfs(u: usize, p: usize, ok: &mut bool, size: &mut Vec<usize>, adj: &Vec<Vec<usize>>, k: usize) {
    size[u] = 1;
    let mut cnt = 0; // number of nonzero children
    for &v in &adj[u] {
        if v != p {
            dfs(v, u, ok, size, adj, k);
            size[u] += size[v];
            if size[v] != 0 {
                cnt += 1;
            }
        }
    }

    // invalid cases
    if size[u] == k && cnt >= 3 {
        *ok = false;
    } else if size[u] > k {
        *ok = false;
    } else if size[u] < k && cnt >= 2 {
        *ok = false;
    }

    // reset size
    if size[u] == k {
        size[u] = 0;
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
