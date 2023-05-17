#![allow(unused)]

fn main() {
    let s = reads();
    let mut cnt = 0;
    let mut i = 0;
    while i < s.len() {
        if i + 1 < s.len() && s[i] == '0' && s[i + 1] == '0' {
            cnt += 1;
            i = i + 2;
        } else {
            cnt += 1;
            i = i + 1;
        }
    }
    println!("{}", cnt);
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
