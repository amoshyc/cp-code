#![allow(unused)]

fn solve() {
    let n = read::<usize>();
    if n == 1 {
        println!("1");
        return;
    }
    if n % 2 == 1 {
        println!("-1");
        return;
    }

    let mut arr = (1..=n).collect::<Vec<usize>>();
    for i in (0..n).step_by(2) {
        arr.swap(i, i + 1);
    }

    println!("{}", join(&arr, " "));
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
