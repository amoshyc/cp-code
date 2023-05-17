#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let arr = readv::<usize>();
    let mut vis = vec![false; n];
    for i in 0..n {
        if !vis[i] {
            vis[arr[i] - 1] = true;
        }
    }
    let ans = (1..=n).filter(|&i| !vis[i - 1]).collect::<Vec<_>>();
    println!("{}", ans.len());
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
