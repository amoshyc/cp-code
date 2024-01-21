#![allow(unused)]

fn main() {
    let n = read::<i64>();

    let mut set = std::collections::HashSet::new();
    for a in (2i64..).take_while(|a| a * a <= n) {
        for b in (2u32..).take_while(|b| a.pow(*b) <= n) {
            set.insert(a.pow(b));
        }
    }
    println!("{}", n - set.len() as i64);
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
