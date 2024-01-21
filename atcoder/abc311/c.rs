#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let nxt = mapv(&readv::<usize>(), |x| *x - 1);

    let mut vis = vec![false; n];
    vis[0] = true;
    let mut u = nxt[0];
    while !vis[u] {
        vis[u] = true;
        u = nxt[u];
    }

    // u is the start of cycle
    let mut cycle = vec![u];
    let mut v = nxt[u];
    while v != u {
        cycle.push(v);
        v = nxt[v];
    }
    cycle.push(u);

    let mut path = cycle.clone();
    path.pop();
    path = mapv(&path, |x| *x + 1);
    println!("{}", path.len());
    println!("{}", join(&path, " "));
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

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
