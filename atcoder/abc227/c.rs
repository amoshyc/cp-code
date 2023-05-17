#![allow(unused)]

fn main() {
    let n = read::<u64>();
    let mut ans = 0;
    for a in (1..).take_while(|a| a * a * a <= n) {
        for b in (a..).take_while(|b| a * b * b <= n) {
            ans += (n / (a * b)) - b + 1;
        }
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
    read::<String>().chars().collect::<Vec<char>>()
}
