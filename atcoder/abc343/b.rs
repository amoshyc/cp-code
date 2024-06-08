#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut adj = vec![];
    for _ in 0..n {
        adj.push(readv::<usize>());
    }
    for i in 0..n {
        let ans: Vec<usize> = (0..n).filter(|&j| adj[i][j] == 1).collect();
        let ans = mapv(&ans, |&x| x + 1);
        println!("{}", join(&ans, " "));
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
