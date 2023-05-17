#![allow(unused)]

use std::io::Write;

fn main() {
    let n = read::<usize>();

    // 0 0 0 1 1 1
    let mut lb = 1;
    let mut ub = n;
    while ub - lb > 1 {
        let m = (lb + ub) / 2;
        println!("? {}", m);
        std::io::stdout().flush();

        let x = read::<usize>();
        if x == 0 {
            lb = m;
        } else {
            ub = m;
        }
    }
    println!("! {}", lb);
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
