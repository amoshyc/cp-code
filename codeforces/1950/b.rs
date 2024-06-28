#![allow(unused)]

fn solve() {
    let n = read::<usize>();
    let mut ans = vec![vec!['.'; 2 * n]; 2 * n];
    for r in 0..(2 * n) {
        for c in 0..(2 * n) {
            if r % 4 <= 1 && c % 4 <= 1 {
                ans[r][c] = '#';
            }
            if r % 4 >= 2 && c % 4 >= 2 {
                ans[r][c] = '#';
            }
        }
    }
    for r in 0..ans.len() {
        println!("{}", join(&ans[r], ""));
    }
}

fn main() {
    let tc = read::<usize>();
    for _ in 0..tc {
        solve();
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
