#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let s = reads();

    let pos = (0..n).filter(|&i| s[i] == '1').collect::<Vec<_>>();
    let p = pos.len() / 2;
    let m = pos[p];

    let mut ans = 0;
    for (i, q) in pos[..p].iter().rev().enumerate() {
        ans += (m - 1 - i) - q;
    }
    for (i, q) in pos[p + 1..].iter().enumerate() {
        ans += q - (m + 1 + i);
    }
    println!("{}", ans);
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
