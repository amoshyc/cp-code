#![allow(unused)]

fn main() {
    let w = read::<usize>();
    let mut ans = vec![];
    for i in 1..=99 {
        ans.push(i);
        ans.push(i * 100);
        ans.push(i * 10000);
    }
    ans.push(1_000_000);
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
