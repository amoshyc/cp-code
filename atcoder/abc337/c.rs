#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let arr = readv::<i32>();

    let inf = 1_000_000_000;
    let mut nxt = vec![inf; n];
    let mut root = inf;
    for i in 0..n {
        if arr[i] == -1 {
            root = i;
        } else {
            nxt[arr[i] as usize - 1] = i;
        }
    }

    let mut ans = vec![root + 1];
    while nxt[root] != inf {
        ans.push(nxt[root] + 1);
        root = nxt[root];
    }

    println!("{}", join(&ans, " "));
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
